

use std::convert::TryFrom;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use utils::{verify, checkArgs};


pub mod sign;
pub mod utils;
pub mod crypter;
pub mod internal;
pub mod view;
pub mod owner;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Popula {
    owner_id: AccountId,
    public_key: String,
    secret_key: Vec<u8>,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct AccessInfo {
    token_id: AccountId,
    amount_to_access: U128
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

#[near_bindgen]
impl Popula {

    #[init]
    pub fn new(public_key: String) -> Self {
        let mut secret_key: Vec<u8> = Vec::new();
        while env::used_gas() < env::prepaid_gas() / 2 {
            let gas_str = env::used_gas().to_string().as_bytes().to_vec();
            let block_index = env::block_height().to_string().as_bytes().to_vec();
            secret_key = [secret_key, gas_str, block_index].concat();
            secret_key = env::sha256(&secret_key);
        }

        Self {
            owner_id: env::predecessor_account_id(),
            public_key: public_key,
            secret_key: secret_key,
        }
    }

    pub fn add_post(&mut self, text: Option<String>, imgs: Option<Vec<String>>, video: Option<String>, audio: Option<String>) {
        checkArgs(text, imgs, video, audio);
    }

    pub fn add_encrypt_post(&mut self, encrypt_string: String, access: Vec<Vec<AccessInfo>>) {
        assert!(access.len() > 0 && access[0].len() > 0, "access can not be empty");
        let args = self.decrypt(encrypt_string);
        checkArgs(args.text, args.imgs, args.video, args.audio);
    }

    pub fn follow(&mut self, account_id: AccountId) {
    }

    pub fn unfollow(&mut self, account_id: AccountId) {
    }

    pub fn like(&mut self, receipt_id: String) {
        Base58CryptoHash::try_from(receipt_id).unwrap();
    }

    pub fn unlike(&mut self, receipt_id: String) {
        Base58CryptoHash::try_from(receipt_id).unwrap();
    }
}
