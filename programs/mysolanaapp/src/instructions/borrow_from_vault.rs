use anchor_lang::{prelude::*, solana_program::system_instruction};


use crate::state::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Token, TokenAccount, transfer, Transfer, Mint},
};

// 1 SOL = 20 USDC
pub const SOL_TO_USDC_RATE: f64 = 20.0;


#[derive(Accounts)]
#[instruction(vault_stable_token_mint_address: Pubkey)]
pub struct BorrowFromVault<'info> {

    // Token program
    pub token_program: Program<'info, Token>,

    // Borrower stable token account
    #[account(mut)]
    pub borrower_stable_token_account: Account<'info, TokenAccount>,

    // Borrower sol token account
    #[account(mut)]
    pub borrower: Signer<'info>,

    // Vault account
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    // Vault stable token account
    #[account(mut)]
    pub vault_stable_token_account: Account<'info, TokenAccount>,

    // Stable token mint
    #[account(
        address = vault_stable_token_mint_address
    )]
    pub vault_stable_token_mint: Account<'info, Mint>,

    // Loan account
    #[account(
        init,
        payer = borrower,
        space = 64,
        seeds = [borrower.key().as_ref(), vault.key().as_ref()],
        bump
    )]
    pub loan: Account<'info, Loan>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(
    ctx: Context<BorrowFromVault>,
    borrow_amount: u64,
    repayment_plan: u64,
) -> Result<()> {
    let vault = &mut ctx.accounts.vault;
    let loan = &mut ctx.accounts.loan;

    // Check if the vault is allowed to be withdrawn from
    // if !vault.allow_withdrawal.contains(&from.key) {
    //     return Err(ErrorCode::Unauthorized.into());
    // }

    // Create transfer instruction
    let transfer_instruction = system_instruction::transfer(
        ctx.accounts.borrower.to_account_info().key,
        ctx.accounts.vault.to_account_info().key,
        borrow_amount
    );

    // Invoke transfer instruction
    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.borrower.to_account_info(),
            ctx.accounts.vault.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    // Add transfer of stable tokens interest

    // Convert sol to stable token equivalent
    let stable_token_amount = (borrow_amount as f64) * SOL_TO_USDC_RATE;

    // Transfer stable tokens from vault to from_token_account
    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_stable_token_account.to_account_info(),
            to: ctx.accounts.borrower_stable_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        }
    );
    transfer(cpi_ctx, stable_token_amount)?;

    // Update vault stable token amount
    vault.stable_amount -= stable_token_amount as u64;
    vault.rewards += interest;

    // Get repayment period
    let now_ts = Clock::get().unwrap().unix_timestamp;
    let borrow_end_time = vault.borrow_end_time;
    let repayment_period = (borrow_end_time - now_ts) / repayment_plan;
    let repayment_amount = stable_token_amount / repayment_plan as f64;
    let repayment_dates = (1..repayment_plan+1).map(|i| now_ts + (i * repayment_period)).collect::<Vec<i64>>();

    // Update loan
    loan.loan_owner = *ctx.accounts.borrower.key;
    loan.sol_amount = borrow_amount;
    loan.sol_price = SOL_TO_USDC_RATE;
    loan.remaining_stable_amount = stable_token_amount;
    loan.repayment_dates = repayment_dates;
    loan.repayment_amount = repayment_amount;
    loan.interest_paid = interest;

    Ok(())
}