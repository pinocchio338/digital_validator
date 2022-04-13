import getGummyrollCrudProgram from "../anchor_programs/getGummyrollCrudProgram";
import getGummyrollCrudAuthorityPDA from "../anchor_programs/pdas/getGummyrollCrudAuthorityPDA";
import * as anchor from "@project-serum/anchor";
import getProofForAsset from "../loaders/getProofForAsset";
import GummyrollProgramId from "../anchor_programs/GummyrollProgramId";

export default async function transferAsset(
  treeAccount: anchor.web3.PublicKey,
  treeAdmin: anchor.web3.PublicKey,
  data: Buffer,
  index: number,
  owner: anchor.web3.PublicKey,
  newOwner: anchor.web3.PublicKey
) {
  const program = getGummyrollCrudProgram();
  const [authorityPda] = await getGummyrollCrudAuthorityPDA(
    treeAccount,
    treeAdmin
  );
  const { proof, root } = await getProofForAsset(treeAccount, index);
  await program.methods
    .transfer(root, data, index)
    .accounts({
      authority: treeAdmin,
      authorityPda,
      merkleRoll: treeAccount,
      owner,
      newOwner,
      gummyrollProgram: GummyrollProgramId,
    })
    .remainingAccounts(
      proof.map((pathPart) => ({
        pubkey: new anchor.web3.PublicKey(pathPart),
        isSigner: false,
        isWritable: false,
      }))
    )
    .rpc({ commitment: "confirmed" });
}
