use anchor_lang::prelude::*;

declare_id!("PmGYVj3SdRb1E84NLfpTExiAQxJ7JYTGQg2C1xpcT1C");

#[program]
pub mod wordchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
