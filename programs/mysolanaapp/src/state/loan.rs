use anchor_lang::prelude::*;

#[account]
pub struct Loan {
    // The account that owns the loan.
    pub loan_owner: Pubkey,
    // Sol amount given to the loan
    pub sol_amount: f64,
    // Sol price at the time of loan
    pub sol_price: f64,
    // The amount of stable tokens remaining to be paid back.
    pub remaining_stable_amount: f64,
    // Repayment dates
    pub repayment_dates: Vec<u64>,
    // Repayment amount
    pub repayment_amount: f64,
    // Interest paid
    pub interest_paid: f64,
}