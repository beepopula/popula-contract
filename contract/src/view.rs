use std::collections::HashMap;

use crate::*;
use utils::get_content_hash;
use post::Hierarchy;

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

    pub fn check_shared(&self, hierarchies: Vec<Hierarchy>, account_id: AccountId) -> bool {
        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies.clone(), &self.encryption_bloom_filter).expect("content not found")
        };
        let share_hash = env::sha256(&(account_id.to_string() + "shared" + &hierarchy_hash).into_bytes());
        let share_hash: CryptoHash = share_hash[..].try_into().unwrap();
        self.relationship_bloom_filter.check(&WrappedHash::from(share_hash))
    }

    pub fn check_viewed(&self, hierarchies: Vec<Hierarchy>, inviter_id: AccountId, account_id: AccountId) -> bool {
        let hierarchy_hash = match get_content_hash(hierarchies.clone(), &self.public_bloom_filter) {
            Some(v) => v,
            None => get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found")
        };
        let view_hash = env::sha256(&(account_id.to_string() + "viewed" + &hierarchy_hash + "invited_by" + &inviter_id.to_string()).into_bytes());
        let view_hash: CryptoHash = view_hash[..].try_into().unwrap();
        self.relationship_bloom_filter.check(&WrappedHash::from(view_hash))
    }

    pub fn get_reports(&self, account_id: AccountId) -> Vec<Report> {
        let account = match self.reports.get(&account_id) {
            Some(v) => v,
            None => return Vec::new()
        };
        account.values().collect()
    }
    
}