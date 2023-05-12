use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AssociationEngineSettings  {
	#[serde(rename = "enableAssociationEngine")]
	pub enable_association_engine: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
