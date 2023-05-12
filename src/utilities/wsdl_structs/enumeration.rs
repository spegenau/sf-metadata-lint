
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Enumeration {
    #[serde(rename = "value")]
    _value: String,
}