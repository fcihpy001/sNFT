use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::{Tx, TxAction};
use cosmwasm_std::{Api, BlockInfo, CanonicalAddr, ReadonlyStorage, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use serde::de::DeserializeOwned;

pub static CONFIG_KEY: &[u8] = b"config";

pub struct Config {
    pub name: String,
    pub symbol: String,
    pub admin: CanonicalAddr,
    pub mint_cnt: u32,
    pub tx_cnt: u64,
    pub token_cnt: u32,
    pub status: u8,
    pub token_supply_is_public: bool,
    pub owner_is_public: bool,
    pub sealed_metadata_is_enabled: bool,
    pub unwrap_to_private: bool,
    pub minter_may_update_metadata: bool,
    pub burn_is_enabled: bool,
}

pub enum StoredTxAction {
    Transfer {
        from: CanonicalAddr,
        sender: Option<CanonicalAddr>,
        recipent: CanonicalAddr,
    },
    Mint {
        minter: CanonicalAddr,
        recipient: CanonicalAddr,
    },
    Burn {
        owner: CanonicalAddr,
        burner: Option<CanonicalAddr>,
    },
}
pub struct StoredTx {
    pub tx_id: u64,
    pub block_height: u64,
    pub block_time: u64,
    pub token_id: String,
    pub action: StorageTxAction,
    pub memo: Option<String>,
}
impl StoredTx {
    pub fn into_humanized<A: Api>(self, api: &A) -> StdResult<Tx> {
        let action = match self.action {
            StoredTxAction::Transfer {
                from,
                sender,
                recipent,
            } => {}
            StoredTxAction::Mint { minter, recipient } => TxAction::Mint {
                minter: api.human_address(&minter)?,
                recipient: api.human_address(&recipient)?
            },
            StoredTxAction::Burn { owner, burner } => {
                let bnr = if let Some(b) = burner{
                    Some(api.human_address(&b)?)
                } else {
                    None
                };
                TxAction::Burn {
                    owner: api.human_address(&owner)?,
                    burner: bnr
                }
            }
        };
        let tx = Tx {
            tx_id: self.tx_id,
            block_height: self.block_height,
            block_time: self.block_time,
            token_id: self.token_id,
            action,
            memo: self.memo,
        };
        Ok(tx)
    }
}
pub fn store_transfer<S: Storage>(
    storage: &mut S,
    config: &mut Config,
    block: &BlockInfo,
    token_id: String,
    from: CanonicalAddr,
    sender: Option<CanonicalAddr>,
    recipient: CanonicalAddr,
    memo: Option<String>,
) -> StdResult<()> {
}
pub fn store_mint<S: Storage>(
    storage: &S,
    config: &mut Config,
    block: &BlockInfo,
    token_id: String,
    minter: CanonicalAddr,
    recipient: CanonicalAddr,
    memo: Option<String>,
) -> StdResult<()> {
}
pub fn store_burn<S: Storage>(
    storage: &mut S,
    config: &mut Config,
    block: &BlockInfo,
    token_id: String,
    owner: CanonicalAddr,
    burner: Option<CanonicalAddr>,
) -> StdResult<()> {
}
fn append_tx_for_add<S: Storage>(
    storage: &mut S,
    tx_id: u64,
    address: &CanonicalAddr,
) -> StdResult<()> {
}
pub fn get_txs<A: Api, S: ReadonlyStorage>(
    api: &A,
    storage: &S,
    address: &CanonicalAddr,
    page: u32,
    page_size: u32,
) -> StdResult<(Vec<Tx>, u64)> {
}
pub struct Permission {
    pub address: CanonicalAddr,
    pub expirations: [Option<Expiration>; 3],
}
pub enum PermissionType {
    ViewOwner,
    ViewMetadata,
    Transfer,
}
pub struct AuthList {
    pub address: CanonicalAddr,
    pub tokens: [Vec<u32>; 3],
}
pub struct ReceiveRegistration {}
pub fn save<T: Serialize, S: Storage>(storage: &mut S, key: &[u8], value: &T) -> StdResult<()> {}
pub fn remove<S: Storage>(storage: &mut S, key: &[u8], value: &[t]) -> StdResult<()> {}
pub fn load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<()> {}
pub fn may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
}
pub fn json_load<T: DeserializeOwned, S: ReadonlyStorage>(storage: &S, key: &[u8]) -> StdResult<T> {
}
pub fn json_may_load<T: DeserializeOwned, S: ReadonlyStorage>(
    storage: &S,
    key: &[u8],
) -> StdResult<Option<T>> {
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,
    pub owner: CanonicalAddr,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}
pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
