
use serde::Deserialize;

use super::schema::Schema;
#[derive(Debug, Deserialize, Clone)]
pub struct Types {
    #[serde(rename = "schema")]
    pub _schema: Schema,
}