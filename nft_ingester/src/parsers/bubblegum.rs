use anchor_client::anchor_lang::prelude::Pubkey;
use lazy_static::lazy_static;
use sea_orm::ActiveValue::Set;
use sea_orm::sea_query::ColumnSpec::Default;
use sea_orm::SqlxPostgresConnector;
use solana_sdk::pubkeys;
use sqlx::{PgPool, query};
use {
    crate::{
        events::handle_event,
        parsers::{InstructionBundle, ProgramHandler, ProgramHandlerConfig},
        utils::{filter_events_from_logs, pubkey_from_fb_table},
        error::IngesterError,
    },
    anchor_client::anchor_lang::AnchorDeserialize,
    bubblegum::state::leaf_schema::LeafSchemaEvent,
    flatbuffers::{ForwardsUOffset, Vector},
    plerkle_serialization::transaction_info_generated::transaction_info::{self},
    solana_sdk,
    sqlx::{self, types::Uuid, Pool, Postgres},
    async_trait::async_trait,
};
use bubblegum::state::leaf_schema::{LeafSchema, Version};
use digital_asset_types::{asset, asset_data};
use digital_asset_types::sea_orm_active_enums::{OwnerType, RoyaltyTargetType};
use crate::{get_gummy_roll_events, gummyroll_change_log_event_to_database};

pubkeys!(
    Bubblegum_Program_ID,
    "BGUMzZr2wWfD2yzrXFEWTK2HbdYhqQCP2EZoPEkZBD6o"
);

pub struct BubblegumHandler {
    id: Pubkey,
    storage: Pool<Postgres>,
}

#[async_trait]
impl ProgramHandler for BubblegumHandler {
    fn id(&self) -> Pubkey {
        self.id
    }

    fn config(&self) -> &ProgramHandlerConfig {
        lazy_static! {
            static ref CONFIG: ProgramHandlerConfig = ProgramHandlerConfig {
                responds_to_instruction: true,
                responds_to_account: false
            };
        }
        return &CONFIG;
    }

    async fn handle_instruction(&self, bundle: &InstructionBundle) -> Result<(), IngesterError> {
        handle_bubblegum_instruction(
            &bundle.instruction,
            &bundle.instruction_logs,
            &bundle.keys,
            bundle.message_id,
            &self.storage,
        )
            .await
    }
}

impl BubblegumHandler {
    pub fn new(pool: Pool<Postgres>) -> Self {
        BubblegumHandler {
            id: Bubblegum_Program_ID(),
            storage: pool,
        }
    }
}

fn get_bubblegum_leaf_event(logs: &Vec<&str>) -> Result<LeafSchemaEvent, ()> {
    let event_logs = filter_events_from_logs(logs);
    if event_logs.is_err() {
        println!("Error finding event logs in bubblegum logs");
        return Err(());
    }

    let mut found_event: Option<LeafSchemaEvent> = None;
    for event in event_logs.unwrap() {
        match handle_event::<LeafSchemaEvent>(event) {
            Ok(leaf_event) => {
                if found_event.is_some() {
                    println!("\tOnly expected one leaf event per bubblegum ix");
                    return Err(());
                }
                found_event = Some(leaf_event);
            }
            Err(_) => {
                println!("\tMalformed bubblegum log event data");
                return Err(());
            }
        }
    }

    match found_event {
        Some(leaf_event) => Ok(leaf_event),
        _ => {
            println!("No bubblegum event found in logs");
            Err(())
        }
    }
}

async fn handle_bubblegum_instruction<'a, 'b>(
    instruction: &'a transaction_info::CompiledInstruction<'a>,
    logs: &Vec<&'a str>,
    keys: &Vector<'b, ForwardsUOffset<transaction_info::Pubkey<'b>>>,
    pid: i64,
    pool: &Pool<Postgres>,
) -> Result<(), IngesterError> {
    println!("{:?}", logs);
    let db = SqlxPostgresConnector::from_sqlx_postgres_pool(PgPool(pool));
    let txn = pool.begin().await.unwrap();
    for change_log_event in get_gummy_roll_events(logs)? {
        gummyroll_change_log_event_to_database(
            change_log_event,
            pid,
            pool,
        ).await;
    }
    match bubblegum::get_instruction_type(instruction.data().unwrap()) {
        bubblegum::InstructionName::Transfer => {
            println!("Bubblegum: Transfer");
            // TODO(): insert uuid with new owner with a greater PID
        }
        bubblegum::InstructionName::Mint => {
            println!("Bubblegum: Mint");

            // Retrieve nonce value from the LeafSchemaEvent emitted
            let leaf_event_result = get_bubblegum_leaf_event(logs);
            if leaf_event_result.is_err() {
                println!("Could not find leaf event");
                return Err(IngesterError::ChangeLogEventMalformed);
            };
            let leaf_event = leaf_event_result.unwrap();


            let accounts = instruction.accounts().unwrap();
            let owner = pubkey_from_fb_table(keys, accounts[4] as usize);
            let delegate = pubkey_from_fb_table(keys, accounts[5] as usize);

            let data = instruction.data().unwrap()[8..].to_owned();
            let data_buf = &mut data.as_slice();
            let ix: bubblegum::instruction::Mint =
                bubblegum::instruction::Mint::deserialize(data_buf).unwrap();
            let metadata = ix.message;

            // Printing metadata instruction arguments for debugging
            println!(
                "\tMetadata info: {} {} {} {}",
                &metadata.name,
                metadata.seller_fee_basis_points,
                metadata.primary_sale_happened,
                metadata.is_mutable,
            );

            let inserted = match (ix.version, leaf_event.schema) {
                (Version::V0, LeafSchema::V0 {
                    id,
                    owner,
                    delegate,
                    nonce,
                    ..
                }) => {
                    let data = asset_data::ActiveModel {

                    };

                    let delegate = if owner == delegate {
                        None
                    } else {
                        delegate
                    };
                    let asset = asset::ActiveModel {
                        id: Set(bs58::decode(id).into_vec().unwrap()),
                        owner: Set(bs58::decode(&owner).into_vec().unwrap()),
                        owner_type: Set(OwnerType::Single),
                        delegate: Set(bs58::decode(&delegate).into_vec().unwrap()),
                        frozen: Set(false),
                        supply: Set(1),
                        supply_mint: Set(None),
                        compressed: Set(true),
                        compressible: Set(false),
                        tree_id: Some(Some(ix.merkle_slab)),
                        leaf: Set(Some(leaf_event.schema.to_node().inner)),
                        revision: Set(0), /// Get gummy roll seq
                        royalty_target_type: RoyaltyTargetType::Creators,
                        royalty_target: Set(None),
                        ..Default::default()
                    };
                }
                _ => {
                    Err(IngesterError::NotImplemented)
                }

            };

            // TODO(): insert ALL metadata for NFT so that it can be hashed on-chain
            // sqlx::query(SET_NFT_APPSQL)
            //     .bind()
            //     .bind(&bs58::decode(&delegate).into_vec().unwrap())
            //     .bind(&Uuid::from_u128(leaf_event.nonce))
            //     // revision
            //     .bind(&pid)
            //     .bind(&metadata.name)
            //     .bind(&metadata.symbol)
            //     .bind(&metadata.uri)
            //     .bind(metadata.seller_fee_basis_points as u32)
            //     .bind(metadata.primary_sale_happened)
            //     .bind(metadata.is_mutable)
            //     .execute(pool)
            //     .await
            //     .unwrap();
            println!("\tInserted metadata!");
        }
        // We should probably ignore Redeem & Cancel Redeem
        // Since redeemed voucher is non-transferable, the owner
        // actually technically still owns the metadata
        // i.e. even though gummyroll remove instruction executed in Redeem
        //      we should remove metadata only in the Decompress ix
        //      otherwise, it becomes hard to reinsert data on a CancelRedeem
        bubblegum::InstructionName::Redeem => {
            println!("Bubblegum: Redeem");
            // TODO(): nothing
        }
        bubblegum::InstructionName::CancelRedeem => {
            println!("Bubblegum: CancelRedeem");
            // TODO(): nothing
        }
        bubblegum::InstructionName::Decompress => {
            println!("Bubblegum: Decompress");
            // TODO(): set nonce uuid to a non-queryable state
        }
        _ => println!("Bubblegum: Ignored instruction"),
    }
    txn.commit().await.map_err(|e| {
        IngesterError::StorageWriteError(e.to_string())
    })
}
