use std::convert::TryInto;

use near_contract_standards::non_fungible_token::events::NftMint;
use near_sdk::CryptoHash;

use crate::*;    

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
    options: Option<Vec<Options>>
}

// #[derive(Serialize, Deserialize)]
// #[serde(crate = "near_sdk::serde")]
// #[derive(Debug)]
// pub struct EncryptArgs {
//     text: Option<String>,
//     imgs: Option<String>,
//     video: Option<String>,
//     audio: Option<String>,
//     options: Option<Vec<Options>>
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub enum Options {
    At(AccountId)
}

#[near_bindgen]
impl Popula {
    pub fn add_post(&mut self, args: String) -> Base58CryptoHash {
        let args_obj: Args = serde_json::from_str(&args).unwrap();
        check_args(args_obj.text, args_obj.imgs, args_obj.video, args_obj.audio);
        
        let args = args.clone() + &bs58::encode(env::random_seed()).into_string();
        let hash = env::sha256(&args.clone().into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hash), true);
        let hash = Base58CryptoHash::from(hash);
        // match args_obj.options {
        //     Some(options) => {
        //         options.iter().for_each(|item| {
        //             match item {
        //                 Options::At(account_id) => {
        //                     NftMint { owner_id: &account_id, token_ids: &["at"], memo: None }.emit();
        //                 },
        //             }
        //         });
        //     },
        //     None => todo!()
        // }
        hash
    }

    // pub fn add_encrypt_post(&mut self, encrypt_info: String, sign: String) -> String {
    //     let hash = env::sha256(&encrypt_info.clone().into_bytes());
    //     let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
    //     let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
    //     verify(hash.clone(), sign.into(), pk.into());

    //     let EncryptInfo { content: args, access: _ } = serde_json::from_str(&encrypt_info).unwrap();
    //     check_encrypt_args(args.text, args.imgs, args.video, args.audio);

    //     let encrypt_info = encrypt_info.clone() + &bs58::encode(env::random_seed()).into_string();
    //     let hash = env::sha256(&encrypt_info.clone().into_bytes());
    //     let hash_str = bs58::encode(hash.clone()).into_string();
    //     let hash:[u8;32] = hash[..].try_into().unwrap();
    //     self.encrypt_post_bloom_filter.set(&WrappedHash::from(hash));
    //     hash_str
    // }

    pub fn like(&mut self, target_hash: Base58CryptoHash) {
        let target_hash = target_hash.try_to_vec().unwrap();
        let target_hash:[u8;32] = target_hash[..].try_into().unwrap();
        assert!(self.public_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");
    }

    pub fn unlike(&mut self, target_hash: Base58CryptoHash) {
        let target_hash = target_hash.try_to_vec().unwrap();
        let target_hash:[u8;32] = target_hash[..].try_into().unwrap();
        assert!(self.public_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");
    }

    pub fn add_comment(&mut self, args: String, target_hash: Base58CryptoHash) -> Base58CryptoHash {
        let target_hash = target_hash.try_to_vec().unwrap();
        let target_hash: [u8;32] = target_hash[..].try_into().unwrap();
        assert!(self.public_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");

        let args_obj: Args = serde_json::from_str(&args).unwrap();
        check_args(args_obj.text, args_obj.imgs, args_obj.video, args_obj.audio);

        let args = args.clone() + &env::block_height().to_string();
        let hash = env::sha256(&args.clone().into_bytes());
        let hash: CryptoHash = hash[..].try_into().unwrap();
        self.public_bloom_filter.set(&WrappedHash::from(hash), true);
        let hash = Base58CryptoHash::from(hash);
        hash
    }

    // pub fn add_encrypt_comment(&mut self, encrypt_info: String, sign: Base58CryptoHash, target_hash: Base58CryptoHash) -> String {
    //     let target_hash = target_hash.try_to_vec().unwrap();
    //     let target_hash: [u8;32] = target_hash[..].try_into().unwrap();
    //     assert!(self.post_bloom_filter.check(&WrappedHash::from(target_hash)), "content not found");

    //     let hash = env::sha256(&encrypt_info.clone().into_bytes());
    //     let sign: Vec<u8> = sign.try_to_vec().unwrap();
    //     let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
    //     verify(hash.clone(), sign.into(), pk.into());

    //     let EncryptInfo { content: args, access: _ } = serde_json::from_str(&encrypt_info).unwrap();
    //     check_encrypt_args(args.text, args.imgs, args.video, args.audio);

    //     let encrypt_info = encrypt_info.clone() + &env::block_height().to_string();
    //     let hash = env::sha256(&encrypt_info.clone().into_bytes());
    //     let hash_str = bs58::encode(hash.clone()).into_string();
    //     let hash:[u8;32] = hash[..].try_into().unwrap();
    //     self.encrypt_post_bloom_filter.set(&WrappedHash::from(hash));
    //     hash_str

    // }
}

    