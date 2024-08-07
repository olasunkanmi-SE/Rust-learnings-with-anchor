use anchor_lang::prelude::*;

declare_id!("EHA4RUUTvAuKqSHuv2UrVnuAxrJdzDDf1CZtRKkhS9HK");

#[program]
pub mod goodlife {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
