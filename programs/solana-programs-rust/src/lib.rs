use anchor_lang::prelude::*;

declare_id!("FcM8MgkzaKgVuu7N4SJb1tbMRZyDG8XCGsiXQRmGkMPb");

#[program]
pub mod solana_programs_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
