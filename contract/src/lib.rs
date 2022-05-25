

use std::convert::{TryFrom, TryInto};

use bloom_filter::{Bloom, WrappedHash};
use near_contract_standards::non_fungible_token::TokenId;
use near_contract_standards::non_fungible_token::metadata::{NFTContractMetadata, TokenMetadata, NFT_METADATA_SPEC};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BlockHeight};
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
            bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1),
        };
        this
    }
}



#[cfg(test)]
mod tests {


}