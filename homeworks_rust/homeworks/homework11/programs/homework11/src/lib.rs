use anchor_lang::prelude::*;
use anchor_lang::error_code;

declare_id!("BYrXxhPXAxoGc4fkYEd7WkecZUnSU4ucj4LKMzGMfqw6");

#[program]
pub mod homework11 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let account: &mut Account<BalanceAccount> = &mut ctx.accounts.balance_account;
        account.balance = 100;
        Ok(())
    }

    pub fn increment_balance(ctx: Context<IncrementBalance>, amount: u64) -> Result<()> {
      let account: &mut Account<BalanceAccount> = &mut ctx.accounts.balance_account;

      if account.balance >= 1000 {
        return Err(ErrorCode::MaxBalanceReached.into());
      }

      if amount % 100 == 0 {
        if account.balance + amount > 1000 {
          return Err(ErrorCode::MaxBalanceInput.into())
        } else {
          account.balance += amount;
          Ok(())
        }
      } else {
        return Err(ErrorCode::IncorrectAmount.into());
      }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
  #[account(init, payer = user, space = 8 + 8)]
  balance_account: Account<'info, BalanceAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct IncrementBalance<'info> {
  #[account(mut)]
    balance_account: Account<'info, BalanceAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[account]
pub struct BalanceAccount {
  pub balance: u64,
}

#[error_code]
pub enum ErrorCode {
  #[msg("Amount must be in denominations of 100")]
  IncorrectAmount,
  #[msg("The maximum balance of 1000 has been reached")]
  MaxBalanceReached,
  #[msg("Amount input exceeds balance maximum, please try a smaller amount")]
  MaxBalanceInput,

}
