

use std::convert::{TryFrom, TryInto};

use bloom_filter::{Bloom, WrappedHash};
use near_contract_standards::non_fungible_token::TokenId;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BlockHeight, CryptoHash};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector, LazyOption};
use utils::{check_args, verify, check_encrypt_args};
use access::Access;


pub mod utils;
pub mod signature;
pub mod bloom_filter;
pub mod post;
pub mod access;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Popula {
    owner_id: AccountId,
    public_key: String,
    public_bloom_filter: Bloom,
    encryption_bloom_filter: Bloom,
    relationship_bloom_filter: Bloom
    // metadata: LazyOption<NFTContractMetadata>,
    // notice_metadata: UnorderedMap<TokenId, TokenMetadata>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldPopula {
    owner_id: AccountId,
    public_key: String,
    bloom_filter: Bloom,
    // metadata: LazyOption<NFTContractMetadata>,
    // notice_metadata: UnorderedMap<TokenId, TokenMetadata>
}

#[near_bindgen]
impl Popula {

    #[init]
    pub fn new(public_key: String) -> Self {
        let mut this = Self {
            owner_id: env::predecessor_account_id(),
            public_key: public_key,
            public_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "public".to_string()),
            encryption_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "encrypt".to_string()),
            relationship_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "relationship".to_string())
        };
        this
    }

    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let prev: OldPopula = env::state_read().expect("ERR_NOT_INITIALIZED");
        assert_eq!(
            env::predecessor_account_id(),
            prev.owner_id,
            "Only owner"
        );

        let this = Popula {
            owner_id: prev.owner_id,
            public_key: prev.public_key,
            public_bloom_filter: prev.bloom_filter,
            encryption_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "encrypt".to_string()),
            relationship_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "relationship".to_string()),
            
        };

        this
    }

    pub fn follow(&mut self, account_id: AccountId) {
        let hash = env::sha256(&(env::predecessor_account_id().to_string() + &account_id.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hash), true);
    }

    pub fn unfollow(&mut self, account_id: AccountId) {
        let hash = env::sha256(&(env::predecessor_account_id().to_string() + &account_id.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hash), false);
    }

    pub fn add_item(&mut self, args: String) -> Base58CryptoHash {
        let hash = env::sha256(&(env::predecessor_account_id().to_string() + &args.clone()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hash), true);
        let hash = Base58CryptoHash::from(hash);
        hash
    }
}



#[cfg(test)]
mod tests {


}