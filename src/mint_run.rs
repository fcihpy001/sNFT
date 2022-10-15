use cosmwasm_std::{Api, CanonicalAddr, HumanAddr, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MintRunInfo {
    pub collection_creator: Option<HumanAddr>,
    pub token_creator: Option<HumanAddr>,
    pub time_of_minting: Option<u64>,
    pub mint_run: Option<u32>,
    pub serial_number: Option<u32>,
    pub quantity_minted_this_run: Option<u32>
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct StoredMintRunInfo {
    pub token_creator: CanonicalAddr,
    pub time_of_minting: u64,
    pub mint_run: Option<u32>,
    pub serial_number: Option<u32>,
    pub quantity_minted_this_run: Option<u32>
}
impl StoredMintRunInfo {
    pub fn to_human<A: Api>(
        &self,
        api: &A,
        contract_creator: HumanAddr
    ) -> StdResult<MintRunInfo> {
        Ok(MintRunInfo {
            collection_creator: Some(contract_creator),
            token_creator: Some(api.human_address(&self.token_creator)?),
            time_of_minting: Some(self.time_of_minting),
            mint_run: self.mint_run,
            serial_number: self.serial_number,
            quantity_minted_this_run: self.quantity_minted_this_run
        })
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SerialNumber {
    pub mint_burn: Option<u32>,
    pub serial_nubmer: u32,
    pub quantity_minted_this_run: Option<u32>
}
