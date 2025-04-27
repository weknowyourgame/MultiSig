use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions, SignerAccount};
use crate::context::CreateTransaction;

pub fn create_transaction(
    ctx: Context<CreateTransaction>,
    accounts: Vec<SignerAccount>,
    data: Vec<u8>,
) -> Result<()> {
    let multisig_account = &mut ctx.accounts.multisig_acnt;
    let transaction_account = &mut ctx.accounts.transaction_account;
    let proposer = &ctx.accounts.proposer;
    
    // Initialize the transaction
    transaction_account.transaction_index = multisig_account.num_transactions_created;
    transaction_account.parent = multisig_account.key();
    transaction_account.initiator = proposer.key();
    transaction_account.accounts = accounts;
    transaction_account.multisig_account = multisig_account.key();
    
    // Initialize all signers as false (not signed yet)
    let signers_count = multisig_account.owners.len();
    let mut signers = vec![false; signers_count];
    
    // Mark the proposer as signed if they are an owner
    if let Some(index) = multisig_account.owners.iter().position(|&owner| owner == proposer.key()) {
        signers[index] = true;
    }
    
    transaction_account.signers = signers;
    transaction_account.time = Clock::get()?.unix_timestamp;
    transaction_account.data = data;
    transaction_account.did_complete = false;
    
    // Increment the transaction counter
    multisig_account.num_transactions_created += 1;
    
    Ok(())
}
