use std::collections::HashMap;

use crate::*;
use post::Hierarchy;


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Drip {
    accounts: LookupMap<AccountId, HashMap<String, U128>>,     //post, comment, subcomment, comment to post, subcomment to post, subcomment to comment, like, report
}

fn set_drip(key: String, map: &mut HashMap<String, U128>) {
    let drip = map.get(&key).unwrap_or(&U128::from(0)).0;
    let drip = U128::from(drip + 1);
    map.insert(key, drip);
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

        for (i, hierarchy) in hierarchies.iter().enumerate() { 
            if hierarchy.account_id == sender_id {
                continue
            }
            let mut account = self.accounts.get(&hierarchy.account_id).unwrap_or(HashMap::new());
            let key = "content".to_string() + &(i + MAX_LEVEL + len - 1).to_string();
            set_drip(key, &mut account);
            self.accounts.insert(&hierarchy.account_id, &account);
        }

        let mut sender = self.accounts.get(&sender_id).unwrap_or(HashMap::new());
        let key = "content".to_string() + &(len).to_string();
        set_drip(key, &mut sender);
        self.accounts.insert(&sender_id, &sender);
    }

    pub fn set_like_drip(&mut self, hierarchies: Vec<Hierarchy>) {
        let account_id = hierarchies.get(hierarchies.len() - 1).unwrap().account_id.clone();
        if account_id == env::signer_account_id() {
            return
        }
        let key = "like".to_string();
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        set_drip(key, &mut account);
        self.accounts.insert(&account_id, &account);
    }

    pub fn set_report_drip(&mut self, account_id: AccountId) {
        if account_id == env::signer_account_id() {
            return
        }
        let key = "report".to_string();
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        set_drip(key, &mut account);
        self.accounts.insert(&account_id, &account);
    }

    pub fn set_report_confirm_drip(&mut self) {
        let account_id = env::signer_account_id();
        let key = "report_confirm".to_string();
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        set_drip(key, &mut account);
        self.accounts.insert(&account_id, &account);
    }

    pub fn set_share_drip(&mut self, account_id: AccountId) {
        if account_id == env::signer_account_id() {
            return
        }
        let key = "share".to_string();
        let mut account = self.accounts.get(&account_id).unwrap_or(HashMap::new());
        set_drip(key, &mut account);
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