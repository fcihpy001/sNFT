use std::fmt::Binary;
use cosmwasm_std::{Coin, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub symbol: String,
    pub admin: Option<HumanAddr>,
    pub entropy: String,
    pub royalty_info: Option<RoyaltyInfo>,
    pub config: Option<InitConfig>,
    pub post_init_callback: Option<PostInitCallback>
}

#[derive(Serialize,Deserialize, JsonSchema, Clone, Debug)]
pub struct InitConfig {
    pub public_token_supply: Option<bool>,
    pub public_owner: Option<bool>,
    pub enable_sealed_metadata: Option<bool>,
    pub unwrapped_metadata_is_private: Option<bool>,
    pub minter_may_update_metadata: Option<bool>,
    pub owner_may_update_metadata: Option<bool>,
    pub enable_burn: Option<bool>
}

impl Default for InitConfig {
    fn default() -> Self {
        InitConfig {
            public_token_supply: Some(false),
            public_owner: Some(false),
            enable_sealed_metadata: Some(false),
            unwrapped_metadata_is_private: Some(false),
            minter_may_update_metadata: Some(true),
            owner_may_update_metadata: Some(false),
            enable_burn: Some(false)
        }
    }
}

pub struct PosInitCallback {
    pub msg: Binary,
    pub contract_address: HumanAddr,
    pub code_hash: String,
    pub send: Vec<Coin>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    MintNft {

    },
    BatchMintNft {

    },
    MintNftClones {

    },
    SetMetadata {

    },
    SetRoyaltyInfo {

    },
    Reveal {

    },
    MakeOwnershipPrivate {

    },
    SetGlobalApproval {

    },
    SetWhiteListedApproval {

    },
    Approve {

    },
    Revoke {

    },
    ApproveAll {

    },
    RevokeAll {

    },
    TransferNft {

    },
    BatchTransferNft {

    },
    SendNft {

    },
    BatchSendNft {

    },
    BurnNft {

    },
    BatchBurnNft {

    },
    RegisterReceiveNft {

    },
    CreateViewingKey {

    },
    SetViewingKey {

    },
    Addminters {

    },
    RemoveMinters {

    },
    SetMinters {

    },
    ChangeAdmin {

    },
    SetContractsStatus {

    },
    RevokePermit {

    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ContractInfo {},
    ContractConfig {},
    Minters {},
    NumTokens {

    },
    AllTokens {

    },
    OwnerOf {

    },
    NftInfo {

    },
    AllNftInfo {

    },
    PrivateMetadata {

    },
    NftDossier {

    },
    BatchNftDossier {

    },
    TokenApprovals {

    },
    InventoryApprovals {

    },
    ApprovedForAll {

    },
    Tokens {

    },
    NumTokensOfOwner {

    },
    IsUnwrapped {

    },
    IsTransferable {

    },
    ImplementTokenSubtype {},
    ImplementsNonTransferableTokens {},
    VerifyTransferApproval {

    },
    TransactionHistory {

    },
    RegisteredCodeHash {

    },
    RoyaltyInfo {

    },
    ContractCreator {},
    WithPermit {

    }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

pub enum AccessLevel {
    ApproveToken,
    All,
    RevokeToken,
    None
}

pub struct Mint {

}

pub struct Burn {

}

pub struct Transfer {

}

pub struct Send {

}

pub enum HandleAnswer {

}

pub struct ViewInfo {

}

pub struct ReciveInfo {

}

pub enum TxAction {

}

pub struct Tx {

}

pub struct Snip721Approval {

}

pub struct Cw721OwnerOfResponse {

}

pub struct BatchNftDossierElemnet {

}

pub enum QueryAnswer {

}

pub enum ResponseStatus {
    Success,
    Failure
}

pub enum ContractStatus {
    Normal,
    StopTranscation,
    StopAll
}

pub enum QueryWithPermit {

}

