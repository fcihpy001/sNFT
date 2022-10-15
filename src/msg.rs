use cosmwasm_std::{Coin, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Binary;
use crate::expiration::Expiration;
use crate::mint_run::SerialNumber;
use crate::token::Metadata;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub name: String,
    pub symbol: String,
    pub admin: Option<HumanAddr>,
    pub entropy: String,
    pub royalty_info: Option<RoyaltyInfo>,
    pub config: Option<InitConfig>,
    pub post_init_callback: Option<PostInitCallback>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct InitConfig {
    pub public_token_supply: Option<bool>,
    pub public_owner: Option<bool>,
    pub enable_sealed_metadata: Option<bool>,
    pub unwrapped_metadata_is_private: Option<bool>,
    pub minter_may_update_metadata: Option<bool>,
    pub owner_may_update_metadata: Option<bool>,
    pub enable_burn: Option<bool>,
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
            enable_burn: Some(false),
        }
    }
}

pub struct PosInitCallback {
    pub msg: Binary,
    pub contract_address: HumanAddr,
    pub code_hash: String,
    pub send: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    MintNft {},
    BatchMintNft {},
    MintNftClones {},
    SetMetadata {},
    SetRoyaltyInfo {},
    Reveal {},
    MakeOwnershipPrivate {},
    SetGlobalApproval {},
    SetWhiteListedApproval {},
    Approve {},
    Revoke {},
    ApproveAll {},
    RevokeAll {},
    TransferNft {},
    BatchTransferNft {},
    SendNft {},
    BatchSendNft {},
    BurnNft {},
    BatchBurnNft {},
    RegisterReceiveNft {},
    CreateViewingKey {},
    SetViewingKey {},
    Addminters {},
    RemoveMinters {},
    SetMinters {},
    ChangeAdmin {},
    SetContractsStatus {},
    RevokePermit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ContractInfo {},
    ContractConfig {},
    Minters {},
    NumTokens {},
    AllTokens {},
    OwnerOf {},
    NftInfo {},
    AllNftInfo {},
    PrivateMetadata {},
    NftDossier {},
    BatchNftDossier {},
    TokenApprovals {},
    InventoryApprovals {},
    ApprovedForAll {},
    Tokens {},
    NumTokensOfOwner {},
    IsUnwrapped {},
    IsTransferable {},
    ImplementTokenSubtype {},
    ImplementsNonTransferableTokens {},
    VerifyTransferApproval {},
    TransactionHistory {},
    RegisteredCodeHash {},
    RoyaltyInfo {},
    ContractCreator {},
    WithPermit {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AccessLevel {
    ApproveToken,
    All,
    RevokeToken,
    None,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Mint {
    pub token_id: Option<String>,
    pub owner: Option<HumanAddr>,
    pub public_metadata: Option<Metadata>,
    pub private_metadata: Option<Metadata>,
    pub serial_number: Option<SerialNumber>,
    pub royalty_info: Option<RoyaltyInfo>,
    pub transferable: Option<bool>,
    pub memo: Option<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Burn {
    pub token_ids: Vec<String>,
    pub memo: Option<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Transfer {
    pub recipient: HumanAddr,
    pub token_ids: Vec<String>,
    pub memo: Option<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Send {
    pub contract: HumanAddr,
    pub receiveer_info: Option<ReciveInfo>,
    pub token_ids: Vec<String>,
    pub msg: Option<Binary>,
    pub memo: Option<String>
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum HandleAnswer {
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
    MakeOwnershipPrivate {

    },
    Reveal {

    },
    Approve {

    },
    Revoke {

    },
    ApproveAll {

    },
    RevokeAll {

    },
    SetGlobalApproval {

    },
    SetWhiteListedApproval {

    },
    TransferNft {

    },
    BatchTransferNft {

    },
    SendNft {

    },
    BatchSendNft {

    },
    RegisterReceiveNft {

    },
    ViewingKey {

    },
    AddMinters {

    },
    RemovedMinters {

    },
    SetMinters {

    },
    SetMiners {

    },
    ChangeAdmin {

    },
    SetContractStatus {

    },
    RevokePermit {

    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ViewInfo {
    pub address: HumanAddr,
    pub viewing_key: String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ReciveInfo {
    pub recipient_code_hash: String,
    pub also_implements_batch_receive_nft: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TxAction {
    Transfer {
        from: HumanAddr,
        sender: Option<HumanAddr>,
        recipient: HumanAddr
    },
    Mint {
        minter: HumanAddr,
        recipient: HumanAddr
    },
    Burn {
        owner: HumanAddr,
        burner: Option<HumanAddr>
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tx {
    pub tx_id: u64,
    pub block_height: u64,
    pub block_time: u64,
    pub token_id: String,
    pub actiion: TxAction,
    pub memo: Option<String>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Snip721Approval {
    pub address: HumanAddr,
    pub view_owner_expiration: Option<Expiration>,
    pub view_private_metadata_expiration: Option<Expiration>,
    pub transfer_expiratioin: Option<Expiration>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cw721OwnerOfResponse {
    pub owner: Option<HumanAddr>,
    pub approval: Vec<Cw721Approval>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BatchNftDossierElemnet {
    pub token_id: String,
    pub owner: Option<HumanAddr>,
    pub public_metadata: Option<Metadata>,
    pub private_metadata: Option<Metadata>,
    pub display_private_metadata_error: Option<String>,
    pub royalty_info: Option<DisplayRoyaltyInfo>,
    pub transferable: bool,
    pub unwrapped: bool,
    pub owner_is_public: bool,
    pub public_ownership_expiration: Option<Expiration>,
    pub private_metadata_is_public: bool,
    pub private_metadata_is_public_expiration: Option<Expiration>,
    pub token_approvals: Option<Vec<Snip721Approval>>,
    pub inventory_approvals: Option<Vec<Snip721Approval>>
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    ContractInfo {

    },
    ContractConfig {

    },
    Minters {

    },
    NumTokens {

    },
    TokenList {

    },
    TokenApprovals {

    },
    InventoryApprovals {

    },
    NftInfo {

    },
    PrivateMetadata {

    },
    AllNftInfo {

    },
    NftDossier {

    },
    BatchNftDossier {

    },
    ApprovedForAll {

    },
    IsUnwrapped {

    },
    IsTransferable {

    },
    ImplementsNonTransferableTokens {

    },
    ImplementsTokenSubtype {

    },
    VerifyTransferApproval {

    },
    TranscationHistory {

    },
    RegisteredCodeHash {

    },
    RoyalyInfo {

    },
    ContractCreator {

    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    Success,
    Failure,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ContractStatus {
    Normal,
    StopTranscation,
    StopAll,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    RoyaltyInfo {},
    PrivateMetadata { token_id: String },
    NftDossier {},
    BatchNftDossier {},
    OwnerOf {},
    AllNftInfo {},
    InventoryApprovals {},
    VerifyTransferApproval {},
    TransactionHistory {},
    NumTokens {},
    AllTokens {},
    TokenApprovals {},
    ApprovalForAll {},
    Tokens {},
    NumTokensOfOwner { owner: HumanAddr },
}
