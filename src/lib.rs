use anchor_lang::prelude::*;
declare_id!("ATWeqjFWbGW463o4d2vYp85CsZYKM5xmLhMAwig8eRGR");
pub mod templates;

#[program]
pub mod yin_and_yang_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
