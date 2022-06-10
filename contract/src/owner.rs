use crate::*;

#[near_bindgen]
impl Popula {
    pub fn get_public_key(&self) -> String {
        self.public_key.clone()
    }

    pub fn set_public_key(&mut self, public_key: String) {
        let sender = env::signer_account_id();
        assert!(sender == self.owner_id, "owner only");
        self.public_key = public_key;
    }
}
