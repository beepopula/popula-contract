use std::collections::HashMap;

use crate::*;

#[near_bindgen]
impl Popula {
    pub fn check_follow(&self, followee: AccountId, follower: AccountId) -> bool {
        let target_hash = env::sha256(&(followee.to_string() + "follwed_by" + &follower.to_string()).into_bytes());
        let target_hash: [u8;32] = target_hash[..].try_into().unwrap();
        self.relationship_bloom_filter.check(&WrappedHash::from(target_hash))
    }

    pub fn get_drip(&self, account_id: AccountId) -> HashMap<String, U128> {
        self.drip.get_drip(account_id)
    }
}