import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Eleven } from "../target/types/eleven";
import { assert } from "chai";

describe("eleven", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Eleven as Program<Eleven>;

  it("It should initialize MyAccount with a value of 100", async () => {
    // Generate a new keypair for the MyAccount
    const myAccount = anchor.web3.Keypair.generate();

    // Execute the initialize function
    await program.methods
      .initialize()
      .accounts({
        myAccount: myAccount.publicKey,
        user: provider.wallet.publicKey, // Include the user (payer) account here
        systemProgram: anchor.web3.SystemProgram.programId, // Ensure the system program is referenced
      })
      .signers([myAccount])
      .rpc();

    // Fetch the account data to check the balance
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check if the balance was set to 100
    assert.equal(account.balance, 100);
  });
});
