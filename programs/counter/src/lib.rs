use anchor_lang::prelude::*;

declare_id!("B5sFkG7nuHi5P98nXwDB4Ter87FZ2ZC9tjkrb6k9NBM5");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize> ) -> Result<()> {
        ctx.accounts.counter.count = 0 ;
        Ok(())
    }
    pub fn increase(ctx: Context<Increment> ) -> Result<()> {
        ctx.accounts.counter.count = ctx.accounts.counter + 1;
        Ok(())
    }
    pub fn decrease(ctx: Context<Decrement> ) -> Result<()> {
        if ctx.accounts.counter.count == 1 {
            return Ok(()) ;
        }
        ctx.accounts.counter.count = ctx.accounts.counter - 1;
        Ok(())
    }

    pub fn set(ctx: Context<Set> , num :u32) -> Result<()> {
        ctx.accounts.counter.count = num;
        Ok(())
    }
    
}

#[account]
pub struct Counter {
    count: u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init ,payer = signer , space = 8 + 4)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
  
}


#[derive(Accounts)]
pub struct Decrement<'info> {
    
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
  
}


#[derive(Accounts)]
pub struct Set<'info> {
    
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub signer: Signer<'info>,
  
}
