use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions};
use crate::context::ExecuteTransaction;

pub fn cancel_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
    let transaction_account = &mut ctx.accounts.transaction_account;
    let signer = &ctx.accounts.payer;
    
    // Only the transaction initiator can cancel it
    require!(transaction_account.initiator == signer.key(), 
        ErrorCode::OnlyInitiatorCanCancel);
    
    // Verify that the transaction has not been executed already
    require!(!transaction_account.did_complete, 
        ErrorCode::TransactionAlreadyExecuted);
    
    // Mark the transaction as completed (canceled)
    transaction_account.did_complete = true;
    
    Ok(())
}
