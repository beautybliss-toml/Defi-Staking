use anchor_lang::prelude::*;

declare_id!("HmXTaac3Cw3xvpfnkMC1nra8QZ1pA2W4c5PQ8HyozHKx");

#[program]
pub mod ohm_fork_defi_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
