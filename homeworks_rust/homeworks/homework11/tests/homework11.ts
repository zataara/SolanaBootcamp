import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Homework11 } from "../target/types/homework11";
import { assert } from "chai";

describe("homework11", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Homework11 as Program<Homework11>;

  it("Is initialized!", async () => {
    // Add your test here.

    const balanceAccount = anchor.web3.Keypair.generate();
    
    // initialize the program
    const tx = await program.methods.initialize()
    .accounts({
      balanceAccount: balanceAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .rpc();

    // get the balance after initialization
    const account = await program.account.balanceAccount.fetch(balanceAccount.publicKey);

    assert.equal(account.balance.toNumber(), 100);

    console.log("Your transaction signature", tx);
  });
});
