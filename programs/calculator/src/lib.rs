use anchor_lang::prelude::*;

declare_id!("ReplaceWithYourProgramId");

#[account]
pub struct NewAccount {
    pub authority: Pubkey,
    pub data: u32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Arithmetic overflow or underflow")]
    MathError,
    #[msg("Only the authority may modify this account")]
    Unauthorized,
}

#[program]
pub mod anchor_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let acct = &mut ctx.accounts.new_account;
        acct.authority = ctx.accounts.signer.key();
        acct.data = 1;

        let rect = Rect { width: 10, height: 20 };
        msg!("Rect: {:?}", rect);
        msg!("Area: {}", rect.area());
        Ok(())
    }

    pub fn double(ctx: Context<Modify>) -> Result<()> {
        only_authority(&ctx.accounts.account, &ctx.accounts.authority)?;
        let x = ctx.accounts.account.data.checked_mul(2).ok_or(ErrorCode::MathError)?;
        ctx.accounts.account.data = x;
        Ok(())
    }

    pub fn halve(ctx: Context<Modify>) -> Result<()> {
        only_authority(&ctx.accounts.account, &ctx.accounts.authority)?;
        ctx.accounts.account.data /= 2;
        Ok(())
    }

    pub fn add(ctx: Context<ModifyWithAmount>, amount: u32) -> Result<()> {
        only_authority(&ctx.accounts.account, &ctx.accounts.authority)?;
        let x = ctx.accounts.account.data.checked_add(amount).ok_or(ErrorCode::MathError)?;
        ctx.accounts.account.data = x;
        Ok(())
    }

    pub fn sub(ctx: Context<ModifyWithAmount>, amount: u32) -> Result<()> {
        only_authority(&ctx.accounts.account, &ctx.accounts.authority)?;
        let x = ctx.accounts.account.data.checked_sub(amount).ok_or(ErrorCode::MathError)?;
        ctx.accounts.account.data = x;
        Ok(())
    }
}

fn only_authority(acct: &Account<NewAccount>, signer: &Signer) -> Result<()> {
    require_keys_eq!(acct.authority, signer.key(), ErrorCode::Unauthorized);
    Ok(())
}

#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + 32 + 4,
        seeds = [b"calc", signer.key().as_ref()],
        bump
    )]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Modify<'info> {
    #[account(
        mut,
        seeds = [b"calc", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub account: Account<'info, NewAccount>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ModifyWithAmount<'info> {
    #[account(
        mut,
        seeds = [b"calc", authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub account: Account<'info, NewAccount>,
    pub authority: Signer<'info>,
}
