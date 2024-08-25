use anchor_lang::prelude::*;

declare_id!("6P97LfDWMxUt4GXFKtYpjJRg2tksgLD2zhpRDx3MeXYp");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World! Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
