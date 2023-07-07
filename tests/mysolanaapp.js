const assert = require("assert");
const anchor = require("@project-serum/anchor");
const spl = require("@solana/spl-token");
const { SystemProgram } = anchor.web3;

describe("mysolanaapp", () => {
  /* create and set a Provider */
  const provider = anchor.getProvider();
  anchor.setProvider(provider);
  const program = anchor.workspace.Mysolanaapp;

  let vaultOwner;
  let depositor;
  let borrower;
  let vaultMintSeed;
  let vaultMintPda;
  let vaultMintPdaBump;
  let vaultAccount;

  before(async () => {
    vaultOwner = await program.provider.wallet.publicKey;
    depositor = await anchor.web3.Keypair.generate();
    borrower = await anchor.web3.Keypair.generate();    
    vaultMintSeed = Buffer.from(anchor.utils.bytes.utf8.encode("vault_mint"));
    [vaultMintPda, vaultMintPdaBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [vaultMintSeed],
      program.programId
    );    
    vaultAccount = await anchor.web3.Keypair.generate();
  });

  it("Initialize the vault", async () => {
    await program.rpc.createVault(
      {
        accounts: {
          from: vaultOwner,
          vault: vaultAccount.publicKey,
          mint: vaultMintPda,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [vaultAccount],
      }
    );
    
    const account = await program.account.vault.fetch(vaultAccount.publicKey);
    // console.log('Vault owner: ', account.owner)
    // console.log('Vault stable amount: ', account.stableAmount.toString())
    // console.log('Vault sol balance: ', account.solAmount.toString())

    assert.ok(account.owner.equals(vaultOwner));
    assert.ok(account.stableAmount.eq(new anchor.BN(0)));
    assert.ok(account.solAmount.eq(new anchor.BN(0)));
  });

  it("deposits 10USDC tokens to the vault", async () => {
  })

  it("borrows 5USDC tokens from the vault", async () => {

  })
});