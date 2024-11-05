use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

use crate::errors::ErrorCode;
use crate::{OHM_FORK_MINT_ADDRESS};

#[derive(Accounts)]
pub struct StakeOhm<'info>{
    /// CHECK
    #[account(
        mut,
        seeds=[b"staking_pda"],
        bump
    )]
    staking_pda: AccountInfo<'info>,

    #[account(
        mut,
        seeds=[b"staking_pda_ata", OHM_FORK_MINT_ADDRESS.as_bytes()],
        token::mint=staking_token_mint,
        token::authority=staking_pda,
        bump
    )]
    staking_pda_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        address=OHM_FORK_MINT_ADDRESS.parse::<Pubkey>().unwrap()
    )]
    staking_token_mint: Account<'info, Mint>,

    #[account(mut)]
    user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    user_info: Account<'info, UserInfo>,

    #[account(mut)]
    user: Signer<'info>,

    system_program: Program<'info, System>,

    token_program: Program<'info, Token>,

    rent: Sysvar<'info, Rent>
}

#[account]
pub struct UserInfo{
    pub amount: u64,
    pub reward: u64,
    pub deposit_slot: u64,
}

pub fn stake_ohm(ctx: Context<StakeOhm>, amount: u64) -> Result<()>{
    require!(amount > 0, ErrorCode::WrongAmount);

    let user_info = &mut ctx.accounts.user_info;

    let clock = Clock::get()?;
    let reward = &mut 0;

    if user_info.amount > 0 {
        let slot = ( clock.slot - user_info.deposit_slot ) / ( 24 * 3600 );
        *reward = user_info.amount / 10 * slot;
    }

    let cpi_accounts = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.staking_pda_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info()
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;

    user_info.amount += amount;
    
    user_info.deposit_slot = clock.slot;

    user_info.reward = *reward;

    Ok(())
}