use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use anchor_lang::system_program;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

use crate::errors::ErrorCode;
use crate::{MAXIMUM_AGE, SOL_USD_FEED_ID, OHM_FORK_MINT_ADDRESS, OHM_FOKK_TOKEN_FEED_ID};

#[derive(Accounts)]
pub struct BuyOhmWithSol<'info>{
    #[account(
        mut,
        seeds=[b"staking_pda"],
        bump
    )]
    staking_pda: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"staking_pda", OHM_FORK_MINT_ADDRESS.as_bytes()],
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

    #[account(
        mut,
        constraint=user_ata.mint==*(staking_token_mint.to_account_info().key) && user_ata.owner==*(user.to_account_info().key)
    )]
    user_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    user: Signer<'info>,

    price_update: Account<'info, PriceUpdateV2>,

    token_program: Program<'info, Token>,

    system_program: Program<'info, System>
}

pub fn buy_ohm_with_sol(ctx: Context<BuyOhmWithSol>, amount: u64) -> Result<()>{
    require!(amount > 0, ErrorCode::WrongAmount);

    let price_update = &mut ctx.accounts.price_update;

    let sol_price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(SOL_USD_FEED_ID)?
    )?;
    let ohm_token_price = price_update.get_price_no_older_than(
        &Clock::get()?,
        MAXIMUM_AGE,
        &get_feed_id_from_hex(OHM_FOKK_TOKEN_FEED_ID)?
    )?;

    let sol_amount = amount * ( ( ohm_token_price.price as u64 ) / ( sol_price.price as u64 ) ) * 70 / 100; /// adjust 25 ~ 30%

    let cpi_ctx1 = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer{
            from: ctx.accounts.user.to_account_info(),
            to: ctx.accounts.staking_pda.to_account_info()
        }
    );

    system_program::transfer(cpi_ctx1, sol_amount);


    let cpi_accounts = Transfer{
        from: ctx.accounts.staking_pda_account.to_account_info(),
        to: ctx.accounts.user_ata.to_account_info(),
        authority: ctx.accounts.staking_pda.to_account_info()
    };

    let bump = ctx.bumps.staking_pda;
    let seeds = &[b"staking_pda".as_ref(), &[bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx2 = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts,
        signer
    );

    token::transfer(cpi_ctx2, amount);


    Ok(())
}

