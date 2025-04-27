use anchor_lang::prelude::*;

#[account]
pub struct MultiSigAccount {
    pub owners: Vec<Pubkey>,
    pub threshold: u8,
    pub nonce: u8,
    pub num_transactions_created: u64,
}

#[account]
pub struct Transactions {
    pub transaction_index: u64,
    pub parent: Pubkey,
    pub initiator: Pubkey,
    pub accounts: Vec<SignerAccount>,
    pub signers: Vec<bool>,
    pub time: i64,
    pub data: Vec<u8>,
    pub did_complete: bool,
    pub multisig_account: Pubkey,
}

#[account]
pub struct SignerAccount {
    pub pubkey: Pubkey,
    pub is_signer: bool
}

impl MultiSigAccount {
    pub fn get_max_size(num_owners: usize) -> usize {
        // Base size
        let base_size = 1 +  // threshold (u8)
                        1 +  // nonce (u8)
                        8;   // num_transactions_created (u64)
        
        // Vector of owners
        let owners_size = 4 +         // vec length prefix
                         (32 * num_owners);
        
        base_size + owners_size
    }
}

impl Transactions {
    pub fn get_max_size(num_signers: usize) -> usize {
        // Base size
        let base_size = 8 +   // transaction_index (u64)
                        32 +  // parent (Pubkey)
                        32 +  // initiator (Pubkey)
                        8 +   // time (i64)
                        1 +   // did_complete (bool)
                        32;   // multisig_account (Pubkey)
        
        let accounts_size = 4 +                              // vec length prefix
                           (num_signers * (32 + 1));         // accounts max * (pubkey + 1 bool)
        
        let signers_size = 4 +                     // vec length prefix
                          num_signers;             // booleans (1 per owner)

        let data_size = 4 +                        // vec length prefix
                       1024;                       // assume max instruction data size of 1KB
        
        base_size + accounts_size + signers_size + data_size
    }
}

impl SignerAccount {
    pub fn get_max_size() -> usize {
        // Base size
        32 +  // pubkey (Pubkey)
        1     // is_signer (bool)
    }
}
