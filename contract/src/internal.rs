use std::convert::TryInto;

use crate::*;

impl Popula {
    pub(crate) fn encrypt(&self, text: String) -> String {
        let encrypt_vec = crypter::encrypt(&self.secret_key, &text.as_bytes().to_vec()).unwrap();
        bs58::encode(encrypt_vec).into_string()
    }

    pub(crate) fn decrypt(&self, text: String) -> Args {
        let encrypt_vec = bs58::decode(text).into_vec().unwrap();
        let decrypt_vec = crypter::decrypt(&self.secret_key, &encrypt_vec).unwrap();
        let args: Args = serde_json::from_slice(&decrypt_vec).unwrap();
        args
    }
}