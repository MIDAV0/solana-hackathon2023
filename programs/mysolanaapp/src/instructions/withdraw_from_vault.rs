use anchor_lang::prelude::*;

use crate::state::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, transfer, Transfer, Burn, burn},
};

#[derive(Accounts)]
#[instruction(vault_stable_token_bump: u8, vault_token_mint_address: Pubkey, stable_token_mint_address: Pubkey)]
pub struct WithdrawFromVault<'info> {

    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        address = vault_token_mint_address,
    )]
    pub vault_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub withdrawer_vault_token_account: Account<'info, TokenAccount>,

    pub withdrawer: Signer<'info>,

    #[account(
        mut,
        seeds = [stable_token_mint.key().as_ref()],
        bump = vault_stable_token_bump,
    )]
    pub vault_stable_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub withdrawer_stable_token_account: Account<'info, TokenAccount>,

    #[account(
        address = stable_token_mint_address,
    )]
    pub stable_token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub vault_account: Account<'info, Vault>,
}

pub fn handler(
    ctx: Context<WithdrawFromVault>,
    stable_token_mint_authority_bump: u8,
    withdraw_amount: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault_account;

    // Check if the vault is initialized
    if !vault.initialized {
        return Err(ErrorCode::VaultNotInitialized.into());
    }

    // Check if the borrowing period has ended

    // Check that the vault is not empty
    if vault.stable_amount == 0 {
        return Err(ErrorCode::VaultEmpty.into());
    }
    let vault_token_supply = vault.vault_token_supply;
    let vault_stable_amount = vault.stable_amount;

    let token_price = vault_stable_amount / vault_token_supply;
    let rewards_share = vault.rewards * withdraw_amount / vault_token_supply;
    let stable_token_withdraw_amount = withdraw_amount * token_price + rewards_share;

    // Check that the vault has enough stable tokens
    if vault.stable_amount < stable_token_withdraw_amount {
        return Err(ErrorCode::VaultNotEnoughStable.into());
    }


    // Burn user's vault tokens
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Burn {
            mint: ctx.accounts.vault_token_mint.to_account_info(),
            from: ctx.accounts.withdrawer_vault_token_account.to_account_info(),
            authority: ctx.accounts.withdrawer.to_account_info(),
        }
    );
    burn(cpi_ctx, withdraw_amount)?;

    // Transfer stable tokens from vault to user
    let stable_mint_address = ctx.accounts.stable_token_mint.key();
    let seeds = &[stable_mint_address.as_ref(), &[stable_token_mint_authority_bump]];
    let signer = [&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_stable_token_account.to_account_info(),
            to: ctx.accounts.withdrawer_stable_token_account.to_account_info(),
            authority: ctx.accounts.vault_stable_token_account.to_account_info(),
        },
        &signer
    );
    transfer(cpi_ctx, stable_token_withdraw_amount)?;

    vault.stable_amount -= stable_token_withdraw_amount;
    vault.vault_token_supply -= withdraw_amount;

    Ok(())
}