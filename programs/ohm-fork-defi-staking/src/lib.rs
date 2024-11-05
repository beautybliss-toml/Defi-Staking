pub mod instructions;
pub mod errors;
pub mod constants;

use anchor_lang::prelude::*;

use instructions::*;
use constants::*;

declare_id!("HmXTaac3Cw3xvpfnkMC1nra8QZ1pA2W4c5PQ8HyozHKx");

#[program]
pub mod lfg_staking {
    use super::*;

    /// Allow users buy ohm fork token with 25 ~ 30% discount using sol
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount` - amount to buy
    ///
    pub fn buy_ohm(ctx: Context<BuyOhmWithSol>, amount: u64) -> Result<()>{
        msg!("Instruction: Buy ohm");
        instructions::buy_ohm_with_sol(ctx, amount)
    }

    /// Admin only initialize ohm fork token staking pool
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    ///
    pub fn ohm_initialize(ctx: Context<OhmInitialize>) -> Result<()> {
        msg!("Instruction: Ohm token Staking Initialize");
        instructions::ohm_initialize(ctx)
    }

    /// Allow user stake ohm token to the staking pool
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount` - amount to stake
    ///
    pub fn stake_ohm(ctx: Context<StakeOhm>, amount: u64) -> Result<()> {
        msg!("Insruction: Stake ohm");
        instructions::stake_ohm(ctx, amount)
    }

    /// Allow users claim reward based their staking amount. (1% reward of day)
    /// 
    /// # Arguments
    ///
    /// * `ctx` - The context of accounts
    /// * `amount` - amount to claim
    ///
    pub fn claim_ohm(ctx: Context<ClaimOhm>, amount: u64) -> Result<()> {
        msg!("Instruction: Claim ohm");
        instructions::claim_ohm(ctx, amount)
    }
}
