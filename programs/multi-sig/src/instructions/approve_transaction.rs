use crate::ErrorCode;
use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions, SignerAccount};
use crate::context::ApproveTransaction;

pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
    let transaction_account = &mut ctx.accounts.transaction_account;
    let multisig_account = &ctx.accounts.multisig_account;
    let signer = &ctx.accounts.payer;
    
    // Verify that the transaction has not been executed already
    require!(!transaction_account.did_complete, ErrorCode::TransactionAlreadyExecuted);
    
    // Find the signer's index in the owners list
    let signer_index = multisig_account.owners
        .iter()
        .position(|owner_pubkey| owner_pubkey == &signer.key())
        .ok_or(ErrorCode::SignerNotInOwners)?;
    
    // Mark the transaction as approved by this signer
    transaction_account.signers[signer_index] = true;
    
    // Save the approval status
    ctx.accounts.signer_account.pubkey = signer.key();
    ctx.accounts.signer_account.is_signer = true;
    
    Ok(())
}
