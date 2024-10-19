use anchor_lang::prelude::*;

declare_id!("ACmnALsktmX88En74HFZYFZktw2yZoGrKs9tfQyG9DgS");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
