import assert from "assert";
import anchor from "@project-serum/anchor";
import * as spl from "@solana/spl-token";
const { SystemProgram, LAMPORTS_PER_SOL } = anchor.web3;
import { User } from "./user.js";



describe("mysolanaapp", () => {
  /* create and set a Provider */
  const provider = anchor.getProvider();
  anchor.setProvider(provider);
  const program = anchor.workspace.Mysolanaapp;
  const connection = anchor.getProvider().connection;

  // Pipeline
  // Create Vault token account for user manually

  let usdcMintAddress;
  let usdcMintKeyPair;
  let vaultMintAddress;
  let vaultMintKeyPair;

  let walletPayer;
  const vaultAccount = anchor.web3.Keypair.generate();


  before(async () => {
    walletPayer = await program.provider.wallet.payer;

    usdcMintKeyPair = await anchor.web3.Keypair.generate();

    usdcMintAddress = await spl.createMint(
      connection,
      walletPayer,
      usdcMintKeyPair.publicKey,
      null,
      1,
      usdcMintKeyPair
    )

    // Vault mint
    vaultMintKeyPair = await anchor.web3.Keypair.generate();
    const [vaultMintPda, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [vaultMintKeyPair.publicKey.toBuffer()],
      program.programId
    );
    vaultMintAddress = await spl.createMint(
      connection,
      walletPayer,
      vaultMintPda,
      null,
      1,
      vaultMintKeyPair
    )
  
  });

  it("Initialize the vault", async () => {
    const [usdcMintPDA, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [usdcMintAddress.toBuffer()],
      program.programId
    );

    const dat = provider.wallet.publicKey;

    await program.rpc.createVault(
      usdcMintAddress,
      {
        accounts: {
          from: provider.wallet.publicKey,
          vault: vaultAccount.publicKey,
          vaultStableTokenAccount: usdcMintPDA,
          stableTokenMint: usdcMintAddress,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
        signers: [vaultAccount],
      }
    );
    
    const account = await program.account.vault.fetch(vaultAccount.publicKey);
    // console.log('Vault owner: ', account.owner)
    // console.log('Vault stable amount: ', account.stableAmount.toString())
    // console.log('Vault sol balance: ', account.solAmount.toString())

    assert.ok(account.owner.equals(program.provider.wallet.publicKey));
    assert.ok(account.stableAmount.eq(new anchor.BN(0)));
    assert.ok(account.solAmount.eq(new anchor.BN(0)));
  });

  it("deposits 10USDC tokens to the vault", async () => {
    const depositor = new User(
      provider.wallet,
      usdcMintAddress,
      vaultMintAddress
    )

    await depositor.getOrCreateUSDCAccount(connection, walletPayer);
    await depositor.getOrCreateVaultAccount(connection, walletPayer);

    const usdcBalance = await depositor.usdcBalance(connection);
    const vaultBalance = await depositor.vaultBalance(connection);
    console.log('USDC balance: ', usdcBalance.toString())
    console.log('Vault balance: ', vaultBalance.toString())

    await spl.mintTo(
      connection,
      walletPayer,
      usdcMintKeyPair.publicKey,
      depositor.usdcTokenAccount,
      usdcMintKeyPair,
      100,
      []
    );

    const [usdcMintPDA, usdcMintBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [usdcMintAddress.toBuffer()],
      program.programId
    );

    const [vaultMintPDA, vaultMintBump] = await anchor.web3.PublicKey.findProgramAddressSync(
      [vaultMintAddress.toBuffer()],
      program.programId
    );

    await program.rpc.depositToVault(
      vaultMintBump,
      usdcMintBump,
      vaultMintAddress,
      usdcMintAddress,
      new anchor.BN(10),
      {
        accounts: {
          tokenProgram: spl.TOKEN_PROGRAM_ID,
          vaultTokenMint: vaultMintAddress,
          vaultTokenMintAuthority: vaultMintPDA,
          depositorVaultTokenAccount: depositor.vaultTokenAccount,
          depositorStableTokenAccount: depositor.usdcTokenAccount,
          depositor: provider.wallet.publicKey,
          vaultStableTokenAccount: usdcMintPDA,
          stableTokenMint: usdcMintAddress,
          vaultAccount: vaultAccount.publicKey,
        },
      },
    );

    const usdcBalanceAfter = await depositor.usdcBalance(connection);
    console.log('USDC balance after: ', usdcBalanceAfter.toString())

    const vaultBalanceAfter = await depositor.vaultBalance(connection);
    console.log('Vault balance after: ', vaultBalanceAfter.toString())
    
  })

  it("borrows 5USDC tokens from the vault", async () => {

  })
});