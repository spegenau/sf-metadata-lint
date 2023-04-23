use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Role  {
	#[serde(rename = "parentRole")]
	pub parent_role: Option<String>,
}
