use anchor_lang::prelude::*;

declare_id!("AKdCgQfijFLT1nhswBUmrfXn8BBezzfwNHEtemn7ePmj");

#[program]
pub mod staking_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    
    pub fn stake(ctx: Context<Stake> ,num :u32) -> Result<()> {
        ctx.accounts.amount.amount = num;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake> ,num :u32) -> Result<()> {
        ctx.accounts.amount.amount -= num;
        Ok(())
    }
    
}

#[account] 

pub struct Amount{
    amount : u64
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer : Signer<'info>,
    #[account(
        init
        bump
        payer = payer
        space = 8 + 32 + 8 + 1 
        seeds = [b"client1" , payer.key().as_ref()]
    )]
    pub pda_account :Account<'info,StakeAccount>,
    pub system_program : Program<'info,System>  
}

#[derive(Accounts)]


pub struct Stake<'info> {
    #[account(mut)]
    pub payer : Signer<'info>,
    pub amount :Account<'info, Amount> 
}

#[derive(Accounts)]


pub struct UnStake<'info> {
    #[account(mut)]
    pub payer : Signer<'info>,
    pub amount :Account<'info, Amount> 
}
