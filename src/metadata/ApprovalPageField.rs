use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ApprovalPageField  {
	#[serde(rename = "field")]
	pub field: Option<Vec<String>>,
}
