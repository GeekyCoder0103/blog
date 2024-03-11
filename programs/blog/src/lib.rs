use anchor_lang::prelude::*;

declare_id!("EZ6TPkmi1fpKmN5z6EcmUvkaVFMF8YfLHKCEbNJazuMR");

#[program]
pub mod blog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
