use anchor_lang::prelude::*;

declare_id!("BPESfSzUuxqyPX1pXz7PQcYtkGZ4s76W6DZYVUufZ7KE");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
