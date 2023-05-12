
use serde::Deserialize;

use super::sequence::Sequence;
#[derive(Debug, Deserialize, Clone)]
pub struct Extension {
    #[serde(rename = "base")]
    pub _extension: Option<String>,

    #[serde(rename = "sequence")]
    pub _sequence: Option<Sequence>,
}