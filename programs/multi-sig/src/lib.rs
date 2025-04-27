use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions, SignerAccount};
use crate::context::ApproveTransaction;

mod state;
mod context;
mod instructions;

use instructions::*;
use context::*;
use state::*;

declare_id!("4Y4g4JbDRHdv8bzykPhQHjCsn3BgNpsySgfjyEx5EAid");

#[program]
pub mod multi_sig {
    use super::*;

    pub fn create_multisig(
        ctx: Context<CreateMultisig>, 
        owners: Vec<Pubkey>, 
        threshold: u8
    ) -> Result<()> {
        instructions::create_multisig::create_multisig(ctx, owners, threshold)
    }

    pub fn create_transaction(
        ctx: Context<CreateTransaction>,
        accounts: Vec<SignerAccount>,
        data: Vec<u8>
    ) -> Result<()> {
        instructions::create_transaction::create_transaction(ctx, accounts, data)
    }

    pub fn approve_transaction(ctx: Context<ApproveTransaction>) -> Result<()> {
        instructions::approve_transaction::approve_transaction(ctx)
    }

    pub fn reject_transaction(ctx: Context<RejectTransaction>) -> Result<()> {
        instructions::reject_transaction::reject_transaction(ctx)
    }

    pub fn cancel_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        instructions::cancel_transaction::cancel_transaction(ctx)
    }

    pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
        instructions::execute_transaction::execute_transaction(ctx)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Threshold must be greater than 0")]
    ThresholdTooLow,
    
    #[msg("Threshold must be less than or equal to the number of owners")]
    ThresholdTooHigh,
    
    #[msg("The signer is not in the owners list")]
    SignerNotInOwners,
    
    #[msg("Transaction has already been executed")]
    TransactionAlreadyExecuted,
    
    #[msg("Only the initiator can cancel the transaction")]
    OnlyInitiatorCanCancel,
    
    #[msg("Not enough approvals to execute the transaction")]
    NotEnoughApprovals,
}
