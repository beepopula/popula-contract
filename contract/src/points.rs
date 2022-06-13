use crate::*;
use post::Hierarchy;

const POINTS_TYPE_COUNT: usize = 8;

#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Debug)]
pub struct Points {
    accounts: LookupMap<AccountId, Vec<U128>>     //post, comment, subcomment, comment to post, subcomment to post, subcomment to comment, report, report_confirm
}

fn fill_vec(vec: &mut Vec<U128>) {
    if vec.len() < POINTS_TYPE_COUNT {
        for _ in 0..POINTS_TYPE_COUNT - vec.len() {
            vec.push(U128::from(0));
        }
    }
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
        let mut sender = self.accounts.get(&sender_id).unwrap_or(vec![U128::from(0); POINTS_TYPE_COUNT]);

        fill_vec(&mut sender);

        for (i, hierarchy) in hierarchies.iter().enumerate() { 
            let mut account = self.accounts.get(&hierarchy.account_id).unwrap();
            fill_vec(&mut account);
            let new_points = &U128::from(0 as u128);
            let mut points = account.get(i + MAX_LEVEL + len).unwrap_or(new_points).clone();
            points = U128::from(u128::from(points) + 1);
            account.insert(i + MAX_LEVEL + len, points);
            self.accounts.insert(&hierarchy.account_id, &account);
        }

        let new_points = &mut U128::from(0 as u128);
        let mut points = sender.get(len).unwrap_or(new_points).clone();
        points = U128::from(u128::from(points) + 1);
        sender.insert(1 + len, points);
        self.accounts.insert(&sender_id, &sender);
    }

    pub fn get_points(&self, account_id: AccountId) -> Vec<U128> {
        let account = self.accounts.get(&account_id).unwrap_or(Vec::new());
        account
    }
}