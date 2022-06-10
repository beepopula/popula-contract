use std::convert::TryInto;

use near_sdk::CryptoHash;

use crate::*;    
use utils::{get_hash_prefix, set_content};

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// #[derive(Debug)]
// pub struct EncryptInfo {
//     content: EncryptArgs,
//     access: Access
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Args {
    text: Option<String>,
    imgs: Option<Vec<String>>,
    video: Option<String>,
    audio: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct EncryptArgs {
    text: Option<String>,
    imgs: Option<String>,
    video: Option<String>,
    audio: Option<String>
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug, Clone)]
pub struct Hierarchy {
    pub target_hash: Base58CryptoHash,
    pub account_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub enum Options {
    At(AccountId)
}

#[near_bindgen]
impl Popula {
    pub fn add_content(&mut self, args: String, hierarchies: Vec<Hierarchy>, options: Option<Vec<Options>>) -> Base58CryptoHash {
        let args_obj: Args = serde_json::from_str(&args).unwrap();
        check_args(args_obj.text, args_obj.imgs, args_obj.video, args_obj.audio);

        assert!(hierarchies.len() <= MAX_LEVEL, "error");

        let hash_prefix = get_hash_prefix(hierarchies.clone(), &self.public_bloom_filter);
        let target_hash = set_content(args, env::signer_account_id(), hash_prefix, &mut self.public_bloom_filter);

        self.points.set_points(hierarchies);
        target_hash
    }

    pub fn add_encrypt_content(&mut self, encrypt_args: String, access: Access, hierarchies: Vec<Hierarchy>, options: Option<Vec<Options>>, nonce: String, sign: String) -> Base58CryptoHash {
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();

        let hash = env::sha256(&(encrypt_args.clone() + &nonce).into_bytes());
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        verify(hash.clone(), sign.into(), pk);

        let args: EncryptArgs = serde_json::from_str(&encrypt_args).unwrap();
        check_encrypt_args(args.text, args.imgs, args.video, args.audio);

        assert!(hierarchies.len() <= MAX_LEVEL, "error");

        let hash_prefix = get_hash_prefix(hierarchies, &self.encryption_bloom_filter);

        let target_hash = set_content(encrypt_args, env::signer_account_id(), hash_prefix, &mut self.encryption_bloom_filter);

        target_hash
    }

    pub fn like(&mut self, target_hash: Base58CryptoHash) {
        let target_hash = target_hash.try_to_vec().unwrap();
        let target_hash:[u8;32] = target_hash[..].try_into().unwrap();
        assert!(self.public_bloom_filter.check(&WrappedHash::from(target_hash)) || self.encryption_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");
    }

    pub fn unlike(&mut self, target_hash: Base58CryptoHash) {
        let target_hash = target_hash.try_to_vec().unwrap();
        let target_hash:[u8;32] = target_hash[..].try_into().unwrap();
        assert!(self.public_bloom_filter.check(&WrappedHash::from(target_hash)) || self.encryption_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");
    }

    #[payable]
    pub fn report(&mut self, target_hash: Base58CryptoHash) {
        let initial_storage_usage = env::storage_usage();
        let hash = CryptoHash::from(target_hash);
        assert!(self.public_bloom_filter.check(&WrappedHash::from(hash)) || self.encryption_bloom_filter.check(&WrappedHash::from(hash)), "content not found");

        self.reports.insert(&target_hash);
        refund_extra_storage_deposit(env::storage_usage() - initial_storage_usage, 0)
    }
}

    