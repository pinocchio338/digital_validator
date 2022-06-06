/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'
export type GumballMachineHeader = {
  urlBase: number[] /* size: 64 */
  nameBase: number[] /* size: 32 */
  symbol: number[] /* size: 8 */
  sellerFeeBasisPoints: number
  isMutable: number
  retainAuthority: number
  padding: number[] /* size: 4 */
  price: beet.bignum
  goLiveDate: beet.bignum
  mint: web3.PublicKey
  botWallet: web3.PublicKey
  receiver: web3.PublicKey
  authority: web3.PublicKey
  collectionKey: web3.PublicKey
  creatorAddress: web3.PublicKey
  extensionLen: beet.bignum
  maxMintSize: beet.bignum
  remaining: beet.bignum
  maxItems: beet.bignum
  totalItemsAdded: beet.bignum
}

/**
 * @category userTypes
 * @category generated
 */
export const gumballMachineHeaderBeet =
  new beet.BeetArgsStruct<GumballMachineHeader>(
    [
      ['urlBase', beet.uniformFixedSizeArray(beet.u8, 64)],
      ['nameBase', beet.uniformFixedSizeArray(beet.u8, 32)],
      ['symbol', beet.uniformFixedSizeArray(beet.u8, 8)],
      ['sellerFeeBasisPoints', beet.u16],
      ['isMutable', beet.u8],
      ['retainAuthority', beet.u8],
      ['padding', beet.uniformFixedSizeArray(beet.u8, 4)],
      ['price', beet.u64],
      ['goLiveDate', beet.i64],
      ['mint', beetSolana.publicKey],
      ['botWallet', beetSolana.publicKey],
      ['receiver', beetSolana.publicKey],
      ['authority', beetSolana.publicKey],
      ['collectionKey', beetSolana.publicKey],
      ['creatorAddress', beetSolana.publicKey],
      ['extensionLen', beet.u64],
      ['maxMintSize', beet.u64],
      ['remaining', beet.u64],
      ['maxItems', beet.u64],
      ['totalItemsAdded', beet.u64],
    ],
    'GumballMachineHeader'
  )
