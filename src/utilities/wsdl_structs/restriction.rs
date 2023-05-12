
use serde::Deserialize;

use super::enumeration::Enumeration;

#[derive(Debug, Deserialize, Clone)]
pub struct Restriction {
    #[serde(rename = "enumeration")]
    _enumeration: Option<Vec<Enumeration>>,

    #[serde(rename = "base")]
    _base: String,
}
