use anchor_lang::prelude::*
use create::state::{MultiSigAccount, Transactions};

#[derive(Accounts)]
pub struct CreateMultisig<'info> {                   // 'info is a lifetime annotation thing meanns this is the context where accounts are active

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,                               // Account responsible for creating multisig wallet
        space = 8 + MultiSigAccount::get_max_size(), // How much storage is req for the account to store multisig data
        seeds = [b"multisig", payer.key().as_ref()], // Payer's public key
        bump
    )]
    pub multisig_account: Account<'info, MultiSigAccount>,

    pub system_program: Program<'info, System>,
}

#[#[derive(Accounts)]
pub struct CreateTransaction<'info> {

    pub proposer: Signer<'info>,
 
    #[account(mut)]
    pub multisig_acnt: Signer<'info, MultiSignAccount>, // Autorizing it to store all signers' data on chain so basically the account that holds all the funds will also contain data of all signer wallets
    
    #[account(
        init,
        payer = proposer,
        space = 8 + TransactionAccount::get_max_size(),
        seeds = [
            b"transaction",
            multisig_account.key().as_ref(),                            // public key of multisig acnt & total transactions created for that particular multisig wallet
            &multisig_account.num_transactions_created.to_le_bytes()
        ],
        bump
    )]
    pub transaction_account: Account<'info, TransactionAccount>,
    
    pub system_program: Program<'info, System>,

