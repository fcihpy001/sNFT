use crate::state::{may_load, remove, save};
use cosmwasm_std::{CanonicalAddr, ReadonlyStorage, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const PREFIX_INVENTORY_INFO: &[u8] = b"invinfo";
pub const PREFIX_INVENTORY_MAP: &[u8] = b"invmap";
pub const PREFIX_INVENTORY_CELL: &[u8] = b"invcell";
#[derive(Serialize, Deserialize)]
pub struct Cell {
    is_free: bool,
    index: Option<u32>
}
#[derive(Serialize, Deserialize)]
pub struct InventoryInfo {
    pub top: u32,
    pub count: u32,
    pub free: Option<u32>
}

pub struct Inventory {
    pub owner: CanonicalAddr,
    pub info: InventoryInfo
}

impl Inventory {
    pub fn new<S: ReadonlyStorage>(
        storage: &S,
        owner: CanonicalAddr
    ) -> StdResult<Self> {

    }
    pub fn insert<S: Storage>(
        &mut self,
        storage: &mut S,
        token_idx: u32,
        save_info: bool
    ) -> StdResult<()> {

    }

    pub fn remove<S: Storage>(
        &mut self,
        storage: &mut S,
        token_idx: u32,
        save_info: bool
    ) -> StdResult<()> {

    }

    pub fn to_set<S: ReadonlyStorage>(
        &self,
        storage: &S
    ) -> StdResult<HashSet<u32>> {

    }
    pub fn contains<S: ReadonlyStorage>(
        &self,
        storage: &S,
        token_idx: u32
    ) -> StdResult<bool> {

    }
    pub fn owns<S: ReadonlyStorage>(
        storage: &S,
        owner: &CanonicalAddr,
        token_idx: u32
    ) -> StdResult<bool> {

    }
    pub fn save<S: Storage>(&self, storage: &mut S) -> StdResult<()> {

    }
}

pub struct InventoryIter<'a> {
    pub inventory: &'a Inventory,
    pub curr: u32,
    pub retrieve_cnt: u32
}

impl<'a> InventoryIter<'a> {
    pub fn new(inventory: &'a Inventory) -> Self {

    }
    pub fn start_after<>(
        storage: &S,
        inventory: &'a Inventory,
        token_index: u32,
        err_msg: &std
    ) -> StdResult<Self> {

    }

    pub fn next<S: ReadonlyStorage>(&mut self, storage: &s)
        -> StdResult<Option<u32>> {

    }
}
