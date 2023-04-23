use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PublicGroups  {
	#[serde(rename = "publicGroup")]
	pub public_group: Option<Vec<String>>,
}
