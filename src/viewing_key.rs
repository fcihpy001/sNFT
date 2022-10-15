use std::fmt;
use std::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Env;
use crate::rand::{sha_256, Prng};
use crate::utils::{create_hashed_password, ct_slice_compare};

pub const VIEWING_KEY_SIZE: usize = 32;
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct ViewingKey(pub String);

impl ViewingKey {
    pub fn check_viewing_key(&self, hash_pw: &[u8]) -> bool {

    }
    pub fn new(
        env: &Env,
        seed: &[u8],
        entropy: &[u8]
    ) -> Self {

    }
    pub fn to_hashed(&self) -> [u8; VIEWING_KEY_SIZE] {
        create_hashed_password(&self.0)
    }
}

impl fmt::Display for ViewingKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
