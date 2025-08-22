use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer ,Transfer};
declare_id!("6Cn3NiqpBqFavRqahmvqpsWcgmFCi72cD3wNaY4Y9bNC");

#[program]
pub mod cpi_contract {
    use super::*;

    pub fn solana_transfer(ctx: Context<Initialize>, amount :u32) -> Result<()> {
        let from_pub= ctx.accounts.sender.to_account_info();
        let to_pubkey= ctx.accounts.receiver.to_account_info(); 
        let program_id = ctx.accounts.system_program.to_account_info();
         
        let cpi_context = CpiContext::new(program_id, Transfer{
        from: from_pub,
        to: to_pubkey
        },);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    sender : Signer<'info>,
    #[account(mut)]
    receiver : Signer<'info>,
    system_program: Program<'info, System>
}
