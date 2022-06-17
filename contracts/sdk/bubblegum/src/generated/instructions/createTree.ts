/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'

/**
 * @category Instructions
 * @category CreateTree
 * @category generated
 */
export type CreateTreeInstructionArgs = {
  maxDepth: number
  maxBufferSize: number
}
/**
 * @category Instructions
 * @category CreateTree
 * @category generated
 */
export const createTreeStruct = new beet.BeetArgsStruct<
  CreateTreeInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['maxDepth', beet.u32],
    ['maxBufferSize', beet.u32],
  ],
  'CreateTreeInstructionArgs'
)
/**
 * Accounts required by the _createTree_ instruction
 *
 * @property [_writable_] authority
 * @property [_writable_, **signer**] payer
 * @property [**signer**] treeCreator
 * @property [] gummyrollProgram
 * @property [_writable_] merkleSlab
 * @category Instructions
 * @category CreateTree
 * @category generated
 */
export type CreateTreeInstructionAccounts = {
  authority: web3.PublicKey
  payer: web3.PublicKey
  treeCreator: web3.PublicKey
  gummyrollProgram: web3.PublicKey
  merkleSlab: web3.PublicKey
}

export const createTreeInstructionDiscriminator = [
  165, 83, 136, 142, 89, 202, 47, 220,
]

/**
 * Creates a _CreateTree_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category CreateTree
 * @category generated
 */
export function createCreateTreeInstruction(
  accounts: CreateTreeInstructionAccounts,
  args: CreateTreeInstructionArgs
) {
  const { authority, payer, treeCreator, gummyrollProgram, merkleSlab } =
    accounts

  const [data] = createTreeStruct.serialize({
    instructionDiscriminator: createTreeInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: authority,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: payer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: treeCreator,
      isWritable: false,
      isSigner: true,
    },
    {
      pubkey: web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: gummyrollProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: merkleSlab,
      isWritable: true,
      isSigner: false,
    },
  ]

  const ix = new web3.TransactionInstruction({
    programId: new web3.PublicKey(
      'BGUMAp9Gq7iTEuizy4pqaxsTyUCBK68MDfK752saRPUY'
    ),
    keys,
    data,
  })
  return ix
}
