use anchor_lang::prelude::*;

declare_id!("AtUswUKcD5VZf4MdN8rBKeRG9AWPK95PwSHTzhVykrJB");

#[program]
pub mod burry_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
