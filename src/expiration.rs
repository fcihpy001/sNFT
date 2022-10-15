use std::fmt;
use std::fmt::Formatter;
use cosmwasm_std::BlockInfo;

pub enum Expiration {
    AtHeight(u64),
    AtTime(u64),
    Never,
}

impl fmt::Display for Expiration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expiration::AtHeight(height) => write!(f, "expiration height: {}", height),
            Expiration::AtTime(time) => write!(f, "expiration time: {}", time),
            Expiration::Never => write!(f, "expiration: never"),
        }
    }
}

impl Default for Expiration {
    fn default() -> Self {
        Expiration::Never
    }
}

impl Expiration {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        match self {
            Expiration::AtHeight(height) => block.height >= *height,
            Expiration::AtTime(time) => block.time >= *time,
            Expiration::Never => false
        }
    }
}