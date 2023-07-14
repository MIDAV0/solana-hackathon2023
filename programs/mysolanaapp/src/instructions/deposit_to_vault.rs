use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, MintTo, mint_to, transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(vault_token_mint_authority_bump: u8, stable_bump: u8, vault_token_mint_address: Pubkey, stable_token_mint_address: Pubkey)]
pub struct DepositToVault<'info> {

    // Token programm
    pub token_program: Program<'info, Token>,

    // Vault token mint
    #[account(
        mut,
        address = vault_token_mint_address,
    )]
    pub vault_token_mint: Account<'info, Mint>,

    // Vault token mint authority
    /// CHECK: only used as a signing PDA
    #[account(
        seeds = [vault_token_mint_address.as_ref()],
        bump = vault_token_mint_authority_bump,
    )]
    pub vault_token_mint_authority: UncheckedAccount<'info>,

    // Depositor vault token account
    #[account(mut)]
    pub depositor_vault_token_account: Account<'info, TokenAccount>,

    // Depositor stable token account
    #[account(mut)]
    pub depositor_stable_token_account: Account<'info, TokenAccount>,

    // Signer / Depositor
    pub depositor: Signer<'info>,

    // Vault stable token account
    #[account(
        mut,
        seeds = [stable_token_mint_address.as_ref()],
        bump = stable_bump,
    )]
    pub vault_stable_token_account: Account<'info, TokenAccount>,

    // Stable token mint
    #[account(
        address = stable_token_mint_address
    )]
    pub stable_token_mint: Account<'info, Mint>,

    // Vault account
    #[account(mut)]
    pub vault_account: Account<'info, Vault>
}

pub fn handler(
    ctx: Context<DepositToVault>,
    stable_mint_bump: u8,
    vault_token_mint_bump: u8,
    deposit_amount: u64, 
) -> Result<()> {

    let vault = &mut ctx.accounts.vault_account;

    let stable_mint_address = ctx.accounts.stable_token_mint.key();
    let seeds = &[stable_mint_address.as_ref(), &[stable_mint_bump]];
    let signer = [&seeds[..]];

    // Transfer vault token to depositor
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        MintTo {
            mint: ctx.accounts.stable_token_mint.to_account_info(),
            to: ctx.accounts.depositor_vault_token_account.to_account_info(),
            authority: ctx.accounts.vault_token_mint_authority.to_account_info(),
        },
        &signer
    );
    mint_to(cpi_ctx, deposit_amount)?;


    // Transfer stable token to vault
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.depositor_stable_token_account.to_account_info(),
            to: ctx.accounts.vault_stable_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        }
    );
    transfer(cpi_ctx, deposit_amount)?;

    vault.stable_amount += deposit_amount;
    vault.vault_token_supply += deposit_amount;

    Ok(())
}