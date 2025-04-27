use anchor_lang::prelude::*;
use crate::state::{MultiSigAccount, Transactions, SignerAccount};

#[derive(Accounts)]
#[instruction(owners: Vec<Pubkey>, threshold: u8)]
pub struct CreateMultisig<'info> {                   // 'info is a lifetime annotation thing meanns this is the context where accounts are active

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,                               // Account responsible for creating multisig wallet
        space = 8 + MultiSigAccount::get_max_size(owners.len()), // How much storage is req for the account to store multisig data
        seeds = [b"multisig", payer.key().as_ref()], // Payer's public key
        bump
    )]
    pub multisig_account: Account<'info, MultiSigAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateTransaction<'info> {
    #[account(mut)]
    pub proposer: Signer<'info>,
 
    #[account(mut)]
    pub multisig_acnt: Account<'info, MultiSigAccount>, // Autorizing it to store all signers' data on chain so basically the account that holds all the funds will also contain data of all signer wallets
    
    #[account(
        init,
        payer = proposer,
        space = 8 + Transactions::get_max_size(multisig_acnt.owners.len()),
        seeds = [
            b"transaction",
            multisig_acnt.key().as_ref(),
            &multisig_acnt.num_transactions_created.to_le_bytes()
        ],
        bump
    )]
    pub transaction_account: Account<'info, Transactions>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ApproveTransaction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub multisig_account: Account<'info, MultiSigAccount>,
    
    #[account(
        mut,
        has_one = multisig_account
    )]
    pub transaction_account: Account<'info, Transactions>,

    #[account(
        init,
        payer = payer,
        space = 8 + SignerAccount::get_max_size(),
        seeds = [b"approve_signer", payer.key().as_ref(), transaction_account.key().as_ref()],
        bump
    )]
    pub signer_account: Account<'info, SignerAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RejectTransaction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub multisig_account: Account<'info, MultiSigAccount>,
    
    #[account(
        mut,
        has_one = multisig_account
    )]
    pub transaction_account: Account<'info, Transactions>,

    #[account(
        init,
        payer = payer,
        space = 8 + SignerAccount::get_max_size(),
        seeds = [b"reject_signer", payer.key().as_ref(), transaction_account.key().as_ref()],
        bump
    )]
    pub signer_account: Account<'info, SignerAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteTransaction<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub multisig_account: Account<'info, MultiSigAccount>,
    
    #[account(
        mut,
        has_one = multisig_account
    )]
    pub transaction_account: Account<'info, Transactions>,

    pub system_program: Program<'info, System>,
}
