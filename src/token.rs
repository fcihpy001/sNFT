use cosmwasm_std::CanonicalAddr;
use crate::state::Permission;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct Token {
    pub owner: CanonicalAddr,
    pub permissions: Vec<Permission>,
    pub unwrapped: bool,
    pub transferable: bool
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Metadata {
    pub token_uri: Option<String>,
    pub extension: Option<Extension>
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Extension {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub media: Option<Vec<MediaFile>>,
    pub protecte_attributes: Option<Vec<String>>,
    pub token_subtype: Option<String>
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: Option<String>,
    pub value: String,
    pub max_value: Option<String>
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct MediaFile {
    pub file_type: Option<String>,
    pub extension: Option<String>,
    pub authentication: Option<Authentication>,
    pub url: String
}
#[derive(Serialize, Deserialize, JsonSchema, Clone, PartialEq, Debug, Default)]
pub struct Authentication {
    pub key: Option<String>,
    pub user: Option<String>
}