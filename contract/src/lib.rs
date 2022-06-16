

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use bloom_filter::{Bloom, WrappedHash};
// use near_contract_standards::non_fungible_token::TokenId;
// use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128, U64};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BlockHeight, CryptoHash};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector, LazyOption, UnorderedSet};
use drip::Drip;
use post::Report;
use utils::{check_args, verify, check_encrypt_args, set_content};
use access::Access;
use utils::refund_extra_storage_deposit;


pub mod utils;
pub mod signature;
pub mod bloom_filter;
pub mod post;
pub mod access;
pub mod moderator;
pub mod drip;
pub mod view;
pub mod owner;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Popula {
    owner_id: AccountId,
    public_key: String,
    moderators: UnorderedSet<AccountId>,
    public_bloom_filter: Bloom,
    encryption_bloom_filter: Bloom,
    relationship_bloom_filter: Bloom,
    reports: UnorderedMap<AccountId, UnorderedMap<Base58CryptoHash, Report>>,
    drip: Drip
    // metadata: LazyOption<NFTContractMetadata>,
    // notice_metadata: UnorderedMap<TokenId, TokenMetadata>
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldPopula {
    owner_id: AccountId,
    public_key: String,
    moderators: UnorderedSet<AccountId>,
    public_bloom_filter: Bloom,
    encryption_bloom_filter: Bloom,
    relationship_bloom_filter: Bloom,
    reports: UnorderedSet<Base58CryptoHash>,
    drip: Drip
    // metadata: LazyOption<NFTContractMetadata>,
    // notice_metadata: UnorderedMap<TokenId, TokenMetadata>
}

const MAX_LEVEL: usize = 3;

#[near_bindgen]
impl Popula {

    #[init]
    pub fn new(public_key: String) -> Self {
        let mut this = Self {
            owner_id: env::signer_account_id(),
            public_key: public_key,
            moderators: UnorderedSet::new(b'm'),
            public_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "public".to_string()),
            encryption_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "encrypt".to_string()),
            relationship_bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1, "relationship".to_string()),
            reports: UnorderedMap::new(b'r'),
            drip: Drip::new()
        };
        this
    }

    #[init(ignore_state)]
    pub fn migrate() -> Self {


        let prev: OldPopula = env::state_read().expect("ERR_NOT_INITIALIZED");
        let success = env::storage_remove(b"r");
        log!("{:?}", success);
        // assert_eq!(
        //     env::signer_account_id(),
        //     prev.owner_id,
        //     "Only owner"
        // );
        
        let this = Popula {
            owner_id: prev.owner_id,
            public_key: prev.public_key,
            moderators: prev.moderators,
            public_bloom_filter: prev.public_bloom_filter,
            encryption_bloom_filter: prev.encryption_bloom_filter,
            relationship_bloom_filter: prev.relationship_bloom_filter,
            reports: UnorderedMap::new(b'r'),
            drip: prev.drip
        };

        this
    }

    pub fn follow(&mut self, account_id: AccountId) {
        let hash = env::sha256(&(env::signer_account_id().to_string() + "follwed_by" + &account_id.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.relationship_bloom_filter.set(&WrappedHash::from(hash), true);
    }

    pub fn unfollow(&mut self, account_id: AccountId) {
        let hash = env::sha256(&(env::signer_account_id().to_string() + "follwed_by" + &account_id.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.relationship_bloom_filter.set(&WrappedHash::from(hash), false);
    }

    pub fn add_item(&mut self, args: String) -> Base58CryptoHash {
        let args = env::signer_account_id().to_string() + &args.clone();
        let target_hash = set_content(args, env::signer_account_id(), "".to_string(), &mut self.public_bloom_filter);
        self.drip.set_content_drip(Vec::new());
        target_hash
    }

    pub fn collect_drip(&mut self) -> HashMap<String, U128> {
        self.drip.get_and_clear_drip(env::signer_account_id())
    }
}



#[cfg(test)]
mod tests {


}