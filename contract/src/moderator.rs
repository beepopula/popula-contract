use crate::*;

#[near_bindgen]
impl Popula {

    pub fn report_confirm(&mut self, target_hash: Base58CryptoHash, del: bool) {
        let sender = env::signer_account_id();
        assert!(sender != self.owner_id || self.moderators.contains(&sender), "no authorization");

        self.reports.remove(&target_hash);    

        if del == false {
            let target_hash = CryptoHash::from(target_hash);
            self.public_bloom_filter.set(&WrappedHash::from(target_hash), false);
            self.encryption_bloom_filter.set(&WrappedHash::from(target_hash), false);
        }
    }

}
