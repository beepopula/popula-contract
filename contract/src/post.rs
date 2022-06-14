use std::convert::TryInto;

use near_sdk::CryptoHash;

use crate::*;    
use utils::{get_content_hash, set_content};

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


#[near_bindgen]
impl Popula {
    pub fn add_content(&mut self, args: String, hierarchies: Vec<Hierarchy>) -> Base58CryptoHash {
        let args_obj: Args = serde_json::from_str(&args).unwrap();
        check_args(args_obj.text, args_obj.imgs, args_obj.video, args_obj.audio);

        assert!(hierarchies.len() < MAX_LEVEL, "error");

        let hash_prefix = get_content_hash(hierarchies.clone(), &self.public_bloom_filter).expect("content not found");
        let target_hash = set_content(args, env::signer_account_id(), hash_prefix, &mut self.public_bloom_filter);

        self.points.set_content_points(hierarchies);
        target_hash
    }

    pub fn add_encrypt_content(&mut self, encrypt_args: String, access: Access, hierarchies: Vec<Hierarchy>, nonce: String, sign: String) -> Base58CryptoHash {
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();

        let hash = env::sha256(&(encrypt_args.clone() + &nonce).into_bytes());
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        verify(hash.clone(), sign.into(), pk);

        let args: EncryptArgs = serde_json::from_str(&encrypt_args).unwrap();
        check_encrypt_args(args.text, args.imgs, args.video, args.audio);

        assert!(hierarchies.len() < MAX_LEVEL, "error");

        let hash_prefix = get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found");

        let target_hash = set_content(encrypt_args, env::signer_account_id(), hash_prefix, &mut self.encryption_bloom_filter);

        target_hash
    }

    pub fn like(&mut self, hierarchies: Vec<Hierarchy>) {
        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found")
        };
        let hash = env::sha256(&(env::signer_account_id().to_string() + "like" + &hierarchy_hash.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        let exist = self.relationship_bloom_filter.check_and_set(&WrappedHash::from(hash));
        if !exist {
            self.points.set_like_points(env::signer_account_id());
        }
    }

    pub fn unlike(&mut self, hierarchies: Vec<Hierarchy>) {
        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found")
        };

        let hash = env::sha256(&(env::signer_account_id().to_string() + "like" + &hierarchy_hash.to_string()).into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        assert!(self.relationship_bloom_filter.check(&WrappedHash::from(hash)), "illegal");
    }

    #[payable]
    pub fn report(&mut self, hierarchies: Vec<Hierarchy>) {
        let initial_storage_usage = env::storage_usage();
        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found")
        };
        self.reports.insert(&Base58CryptoHash::try_from(hierarchy_hash).unwrap());
        refund_extra_storage_deposit(env::storage_usage() - initial_storage_usage, 0)
    }

    pub fn del_content(&mut self, hierarchies: Vec<Hierarchy>) {
        let sender_id = env::signer_account_id();
        assert!(hierarchies.get(hierarchies.len() - 1).unwrap().account_id == sender_id, "not content owner");

        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found")
        };
        let hierarchy_hash = Base58CryptoHash::try_from(hierarchy_hash).unwrap().try_to_vec().unwrap();
        let hierarchy_hash: CryptoHash = hierarchy_hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hierarchy_hash), false);
    }

    pub fn share_view(&mut self, hierarchies: Vec<Hierarchy>, nonce: String, sign: String) {

    }
}