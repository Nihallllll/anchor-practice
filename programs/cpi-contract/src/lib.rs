use anchor_lang::prelude::*;

declare_id!("6Cn3NiqpBqFavRqahmvqpsWcgmFCi72cD3wNaY4Y9bNC");

#[program]
pub mod cpi_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
