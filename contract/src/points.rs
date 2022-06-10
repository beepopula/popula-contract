use crate::*;
use post::Hierarchy;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Points {
    accounts: LookupMap<AccountId, Vec<U128>>     //post, comment, subcomment, comment to post, subcomment to post, subcomment to comment
}

impl Points {
    pub fn new() -> Self{
        Points { 
            accounts:  LookupMap::new("points".as_bytes())
        }
    }

    pub fn set_points(&mut self, hierarchies: Vec<Hierarchy>) {
        let len = hierarchies.len();
        let sender_id = env::signer_account_id();
        let mut sender = self.accounts.get(&sender_id).unwrap_or(Vec::new());

        for (i, hierarchy) in hierarchies.iter().enumerate() { 
            let mut account = self.accounts.get(&hierarchy.account_id).unwrap();
            let new_points = &U128::from(0 as u128);
            let mut points = account.get(i + MAX_LEVEL + len).unwrap_or(new_points).clone();
            points = U128::from(u128::from(points) + 1);
            account.insert(i + MAX_LEVEL + len, points);
            self.accounts.insert(&hierarchy.account_id, &account);
        }

        let new_points = &mut U128::from(0 as u128);
        let mut points = sender.get(1 + len).unwrap_or(new_points).clone();
        points = U128::from(u128::from(points) + 1);
        sender.insert(1 + len, points);
        self.accounts.insert(&sender_id, &sender);
    }

    pub fn get_points(&self, account_id: AccountId) -> Vec<U128> {
        let account = self.accounts.get(&account_id).unwrap_or(Vec::new());
        account
    }
}