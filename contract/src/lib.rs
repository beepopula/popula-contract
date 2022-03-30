

use std::convert::{TryFrom, TryInto};

use bloom_filter::{Bloom, WrappedHash};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BlockHeight};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
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
    bloom_filter: Bloom
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct AccessInfo {
    token_id: AccountId,
    amount_to_access: U128
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct EncryptInfo {
    content: EncryptArgs,
    access: String
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Args {
    text: Option<String>,
    imgs: Option<Vec<String>>,
    video: Option<String>,
    audio: Option<String>
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct EncryptArgs {
    text: Option<String>,
    imgs: Option<String>,
    video: Option<String>,
    audio: Option<String>
}

#[near_bindgen]
impl Popula {

    #[init]
    pub fn new(public_key: String) -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            public_key: public_key,
            bloom_filter: Bloom::new_for_fp_rate_with_seed(1000000, 0.1)
        }
    }
}



#[cfg(test)]
mod tests {


}