use std::collections::HashMap;

use crate::*;
use post::Hierarchy;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Drip {
    accounts: LookupMap<AccountId, HashMap<String, U128>>,     //post, comment, subcomment, comment to post, subcomment to post, subcomment to comment, like, report
}



impl Drip {
    pub fn new() -> Self{
        Drip { 
            accounts:  LookupMap::new("drip".as_bytes())
        }
    }

    pub fn set_content_drip(&mut self, hierarchies: Vec<Hierarchy>) {
        let len = hierarchies.len();
        let sender_id = env::signer_account_id();
        let mut sender = self.accounts.get(&sender_id).unwrap_or(HashMap::new());

        for (i, hierarchy) in hierarchies.iter().enumerate() { 
            let mut account = self.accounts.get(&hierarchy.account_id).unwrap_or(HashMap::new());
            let key = "content".to_string() + &(i + MAX_LEVEL).to_string();
            let content_drip = account.get(&key).unwrap_or(&U128::from(0)).0;
            
            let content_drip = U128::from(content_drip + 1);
            account.insert(key, content_drip);
            self.accounts.insert(&hierarchy.account_id, &account);
        }

        let key = "content".to_string() + &(len).to_string();
        let content_drip = sender.get(&key).unwrap_or(&U128::from(0)).0;
        let content_drip = U128::from(content_drip + 1);
        sender.insert(key, content_drip);
        self.accounts.insert(&sender_id, &sender);
    }

    pub fn set_like_drip(&mut self, account_id: AccountId) {
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        let drip = account.get(&"like".to_string()).unwrap_or(&U128::from(0)).0;
        let drip = U128::from(drip + 1);
        account.insert("like".to_string(), drip);
        self.accounts.insert(&account_id, &account);
    }

    pub fn get_and_clear_drip(&mut self, account_id: AccountId) -> HashMap<String, U128> {
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        let ret = account.clone();
        for (_, value) in account.iter_mut() {
            *value = U128::from(0);
        }
        self.accounts.insert(&account_id, &account);
        ret
    }

    pub fn get_drip(&self, account_id: AccountId) -> HashMap<String, U128> {
        let account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        account
    }

    
}