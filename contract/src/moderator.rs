use crate::*;
use utils::get_content_hash;
use post::Hierarchy;

#[near_bindgen]
impl Popula {

    pub fn report_confirm(&mut self, hierarchies: Vec<Hierarchy>, del: bool) {
        let sender = env::signer_account_id();
        assert!(sender != self.owner_id || self.moderators.contains(&sender), "no authorization");

        let hierarchy_hash = get_content_hash(hierarchies.clone(), &self.public_bloom_filter).unwrap_or_else(|| get_content_hash(hierarchies, &self.encryption_bloom_filter).expect("content not found"));
        let hierarchy_hash = Base58CryptoHash::try_from(hierarchy_hash).unwrap();
        self.reports.remove(&hierarchy_hash);    

        if del == false {
            let hierarchy_hash = CryptoHash::from(hierarchy_hash);
            self.public_bloom_filter.set(&WrappedHash::from(hierarchy_hash), false);
            self.encryption_bloom_filter.set(&WrappedHash::from(hierarchy_hash), false);
        }
    }

}
