use crate::*;
use utils::get_hash_prefix;
use post::Hierarchy;

#[near_bindgen]
impl Popula {

    pub fn report_confirm(&mut self, hierarchies: Vec<Hierarchy>, del: bool) {
        let sender = env::signer_account_id();
        assert!(sender != self.owner_id || self.moderators.contains(&sender), "no authorization");

        let hierarchy_hash = get_hash_prefix(hierarchies.clone(), &self.public_bloom_filter).unwrap_or(get_hash_prefix(hierarchies, &self.encryption_bloom_filter).expect("content not found"));
        let hierarchy_hash = Base58CryptoHash::try_from(hierarchy_hash).unwrap();
        self.reports.remove(&hierarchy_hash);    

        if del == false {
            let hierarchy_hash = CryptoHash::from(hierarchy_hash);
            self.public_bloom_filter.set(&WrappedHash::from(hierarchy_hash), false);
            self.encryption_bloom_filter.set(&WrappedHash::from(hierarchy_hash), false);
        }
    }

}
