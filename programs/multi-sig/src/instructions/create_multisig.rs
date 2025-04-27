use crate::ErrorCode;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::instruction::Instruction;
use crate::state::MultiSigAccount;
use crate::context::CreateMultisig;

pub fn create_multisig(
    ctx: Context<CreateMultisig>,
    owners: Vec<Pubkey>,
    threshold: u8,
) -> Result<()> {
    let multisig_account = &mut ctx.accounts.multisig_account;
    
    // Validate the threshold
    require!(threshold > 0, ErrorCode::ThresholdTooLow);
    require!(threshold <= owners.len() as u8, ErrorCode::ThresholdTooHigh);
    
    // Initialize the multisig account
    multisig_account.owners = owners;
    multisig_account.threshold = threshold;
    multisig_account.nonce = 0;
    multisig_account.num_transactions_created = 0;
    
    Ok(())
}
