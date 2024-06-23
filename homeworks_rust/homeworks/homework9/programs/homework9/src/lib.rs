use anchor_lang::prelude::*;

declare_id!("ABYafGWY7bkcazkXA5RA4qUmhLThhE7vVJEtdXYztPi8");

#[program]
pub mod homework9 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
