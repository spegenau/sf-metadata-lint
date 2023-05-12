
use serde::Deserialize;

use super::extension::Extension;
#[derive(Debug, Deserialize, Clone)]
pub struct ComplexContent {
    #[serde(rename = "extension")]
    pub extension: Extension,
}
