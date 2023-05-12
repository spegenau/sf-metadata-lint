
use serde::Deserialize;

use super::metadata_type::MetadataType;
#[derive(Debug, Deserialize, Clone)]
pub struct Schema {
    #[serde(rename = "metadataType")]
    pub metadata_types: Vec<MetadataType>,
}