

use std::convert::TryFrom;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{Base58CryptoHash, U128};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, AccountId, log, bs58, PanicOnDefault, Promise, BlockHeight};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use utils::{checkArgs, verify};


pub mod utils;
pub mod signature;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Popula {
    owner_id: AccountId,
    public_key: String
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
pub struct EncryptInfo {
    content: Args,
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

#[near_bindgen]
impl Popula {

    #[init]
    pub fn new(public_key: String) -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            public_key: public_key
        }
    }

    pub fn add_post(&mut self, text: Option<String>, imgs: Option<Vec<String>>, video: Option<String>, audio: Option<String>) {
        checkArgs(text, imgs, video, audio);
    }

    pub fn add_encrypt_post(&mut self, encrypt_info: String, sign: String) {
        let hash = env::sha256(&encrypt_info.clone().into_bytes());
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
        verify(hash, sign.into(), pk.into());

        let EncryptInfo { content: args, access: _ } = serde_json::from_str(&encrypt_info).unwrap();
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
