use anchor_lang::prelude::*;

declare_id!("4Y4g4JbDRHdv8bzykPhQHjCsn3BgNpsySgfjyEx5EAid");

#[program]
pub mod multi_sig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
