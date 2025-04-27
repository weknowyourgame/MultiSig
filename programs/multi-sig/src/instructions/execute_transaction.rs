use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions};
use crate::context::ExecuteTransaction;
use crate::ErrorCode;

pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
    let transaction_account = &mut ctx.accounts.transaction_account;
    let multisig_account = &ctx.accounts.multisig_account;
    
    // Verify that the transaction has not been executed already
    require!(!transaction_account.did_complete, ErrorCode::TransactionAlreadyExecuted);
    
    // Count the number of approvals
    let approval_count = transaction_account.signers.iter().filter(|&&approved| approved).count();
    
    // Check if we have enough approvals
    require!(
        approval_count >= multisig_account.threshold as usize,
        ErrorCode::NotEnoughApprovals
    );
    
    // Mark the transaction as completed
    transaction_account.did_complete = true;
    
    // In a full implementation, you would execute the transaction here
    
    Ok(())
}