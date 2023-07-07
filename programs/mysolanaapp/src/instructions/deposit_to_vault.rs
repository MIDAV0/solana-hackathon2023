use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, MintTo, mint_to, transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64, mint_bump: u8, mint_seed: Vec<u8>)]
pub struct DepositToVault<'info> {
    // Mint to create vault tokens
    #[account(
        mut,
        seeds = [&mint_seed],
        bump = mint_bump,
    )]
    pub mint: Account<'info, Mint>,

    // Vault token account of person who deposited to vault
    #[account(
        init_if_needed,
        payer = from_signer,
        associated_token::mint = mint,
        associated_token::authority = from_signer,
    )]
    pub from_vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, Vault>,

    // Vault stable token account
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    // Person who deposited to vault
    #[account(mut)]
    pub from_signer: Signer<'info>,

    // Stable token account of person who deposited to vault
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<DepositToVault>,
    amount: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let vault_token_account = &mut ctx.accounts.vault_token_account;
    let from = &mut ctx.accounts.from;
    let authority = &ctx.accounts.from_signer;
    let token_program = &ctx.accounts.token_program;
    // let bump = *ctx.bumps.get("vault_mint").unwrap();
    let seeds = [b"vault_mint".as_ref()];
    let signer_seeds = &[&seeds[..]][..];


    // Transfer tokens from the `from` account to the `to` account.
    let cpi_accounts = Transfer {
        from: from.to_account_info().clone(),
        to: vault_token_account.to_account_info().clone(),
        authority: authority.to_account_info().clone(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer(cpi_ctx, amount)?;


    // Update the vault
    vault.stable_amount += amount;

    // Airdrop tokens to the vault token account of the person who deposited to vault
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.from_vault_token_account.to_account_info(),
        authority: ctx.accounts.mint.to_account_info(),
    };
    

    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(
        cpi_program, 
        cpi_accounts,
        signer_seeds,
    );
    mint_to(cpi_ctx, amount)?;

    Ok(())
}