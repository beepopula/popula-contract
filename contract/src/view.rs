use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct EncryptInfo {
    receipt_id: String,
    encrypt_string: String
}


#[near_bindgen]
impl Popula {
    pub fn get_encrypt_post(&self, text: Option<String>, imgs: Option<Vec<String>>, video: Option<String>, audio: Option<String>) -> String {
        checkArgs(text.clone(), imgs.clone(), video.clone(), audio.clone());
        let args_str = json!({
            "text": text,
            "imgs": imgs,
            "video": video,
            "audio": audio
        }).to_string().into();
        self.encrypt(args_str)
    }

    pub fn decrypt_posts(&self, encrypt_info: Vec<EncryptInfo>, sign: String) -> Vec<Args> {
        let json_str = json!(encrypt_info).to_string();
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
        verify(json_str.as_bytes().to_vec(), sign.into(), pk.into());

        let mut ret: Vec<Args> = Vec::new();
        for item in encrypt_info {
            ret.push(self.decrypt(item.encrypt_string));
        }
        ret
    }
}



#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::json_types::ValidAccountId;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());
        let contract = Popula::new("EWGCyGKXrQFQRwKdYCwkTLruv2wPCpA28hvqGjiQCDAv".to_string());
        testing_env!(context.is_view(true).build());
        let obj:Vec<EncryptInfo> = vec![EncryptInfo {
            encrypt_string: "2AMXSaSjrGCpxrqGkaaXfdTLDLB2rGaURht9j38uSrxNHbgfitXyUbbgGZWZnGW7DSkrfztfFi5w28HWD5DaGuNEWEZPRxn2Tan2hwNCaM6ZZibnXHdJMEjWsEJK2TmjRcdZAY4WeYTSvarpYf6NFo4uVs3APWfKhSdjy5Vf1oUtJJi2BRExcwDDucGzCMLAuyXSAEMc1P6NogBMB5u5g5VojCMFNP".to_string(), 
            receipt_id: "5VWcMsVnNKWshKqHq2kBiHMyuqodLb4rDhiho2JFjLuY".to_string()
        }];
        contract.decrypt_posts(obj, "eBa8DhwS7buwFiNUA1aDYZWUugfswG8Rb5NuBBT6LCUqTmzYkMvtHj8GmPuABtVWJVMFsiHCzyD3ccfXc8mAFpe".to_string());
    }
}
