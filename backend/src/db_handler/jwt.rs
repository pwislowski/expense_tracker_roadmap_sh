use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::{collections::BTreeMap, error::Error};

use super::types::User;

pub struct JwtHandler {
    key: Hmac<Sha256>,
}

const KEY_JWT: &'static str = "creds";

impl JwtHandler {
    pub fn sign(self, string: &str) -> Result<String, Box<dyn Error>> {
        let mut claims = BTreeMap::new();
        claims.insert(KEY_JWT, string);
        let token_str = claims.sign_with_key(&self.key)?;

        Ok(token_str)
    }

    pub fn decode(self, token_str: &str) -> Result<User, Box<dyn Error>> {
        let btree_res: BTreeMap<String, String> = token_str.verify_with_key(&self.key)?;

        let creds = btree_res[KEY_JWT].clone();
        let user: User = serde_json::from_str(&creds).unwrap();

        Ok(user)
    }

    pub fn new(secret: String) -> Self {
        let key = Hmac::new_from_slice(secret.as_bytes()).unwrap();
        JwtHandler { key }
    }
}
