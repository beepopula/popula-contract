use std::collections::HashMap;

use crate::*;
use post::Hierarchy;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Points {
    accounts: LookupMap<AccountId, HashMap<String, U128>>,     //post, comment, subcomment, comment to post, subcomment to post, subcomment to comment, like, report
}



impl Points {
    pub fn new() -> Self{
        Points { 
            accounts:  LookupMap::new("points".as_bytes())
        }
    }

    pub fn set_content_points(&mut self, hierarchies: Vec<Hierarchy>) {
        let len = hierarchies.len();
        let sender_id = env::signer_account_id();
        let mut sender = self.accounts.get(&sender_id).unwrap_or(HashMap::new());

        for (i, hierarchy) in hierarchies.iter().enumerate() { 
            let mut account = self.accounts.get(&hierarchy.account_id).unwrap_or(HashMap::new());
            let key = "content".to_string() + &(i + MAX_LEVEL).to_string();
            let content_points = account.get(&key).unwrap_or(&U128::from(0)).0;
            
            let content_points = U128::from(content_points + 1);
            account.insert(key, content_points);
            self.accounts.insert(&hierarchy.account_id, &account);
        }

        let key = "content".to_string() + &(len).to_string();
        let mut content_points = sender.get(&key).unwrap_or(&U128::from(0)).0;
        let content_points = U128::from(content_points + 1);
        sender.insert(key, content_points);
        self.accounts.insert(&sender_id, &sender);
    }

    pub fn set_like_points(&mut self, account_id: AccountId) {
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        let points = account.get(&"like".to_string()).unwrap_or(&U128::from(0)).0;
        let points = U128::from(points + 1);
        account.insert("like".to_string(), points);
        self.accounts.insert(&account_id, &account);
    }

    pub fn get_points(&self, account_id: AccountId) -> HashMap<String, U128> {
        let account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        account
    }
}