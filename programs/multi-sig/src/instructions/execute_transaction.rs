use crate::ErrorCode;
use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions};
use crate::context::ExecuteTransaction;
use anchor_lang::solana_program::{
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
};

pub fn execute_transaction(ctx: Context<ExecuteTransaction>) -> Result<()> {
    let transaction_account = &mut ctx.accounts.transaction_account;
    let multisig_account = &ctx.accounts.multisig_account;
    
    require!(!transaction_account.did_complete, ErrorCode::TransactionAlreadyExecuted);
    
    let approval_count = transaction_account.signers.iter().filter(|&&approved| approved).count();
    
    require!(
        approval_count >= multisig_account.threshold as usize,
        ErrorCode::NotEnoughApprovals
    );
    
    transaction_account.did_complete = true;
    
    let tx_data = &transaction_account.data;
    
    let mut account_metas: Vec<AccountMeta> = Vec::new();
    
    for account in &transaction_account.accounts {
        if account.is_signer {
            account_metas.push(AccountMeta::new(account.pubkey, true));
        } else {
            account_metas.push(AccountMeta::new_readonly(account.pubkey, false));
        }
    }
    
    if account_metas.is_empty() {
        return Ok(());
    }
    
    let program_id = account_metas[0].pubkey;
    
    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts: account_metas,
        data: tx_data.clone(),
    };
    
    // Execute the instruction
    // For now invoke the instruction directly
    anchor_lang::solana_program::program::invoke(
        &instruction,
        &ctx.accounts.to_account_infos(),
    )?;
    
    Ok(())
}