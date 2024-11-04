pub mod instructions;
pub mod errors;
pub mod constants;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("HmXTaac3Cw3xvpfnkMC1nra8QZ1pA2W4c5PQ8HyozHKx");

#[program]
pub mod lfg_staking {
    use super::*;

    // ohm token staking initialize
    pub fn Ohm_fork_initialize(ctx: Context<OhmInitialize>) -> Result<()> {
        msg!("Ohm token staking initialize");
        instructions::ohm_initialize(ctx)
    }
}
