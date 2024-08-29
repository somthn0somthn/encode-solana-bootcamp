use anchor_lang::prelude::*;

declare_id!("EgRFKeAiDc9GjGH1sWaPFzMwh39t3yDP2Fyo8dKNdM4B");

#[program]
pub mod eleven {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.my_account.balance = 100;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct MyAccount {
    pub balance: u64
}