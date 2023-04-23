use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AssociationEngineSettings  {
	#[serde(rename = "enableAssociationEngine")]
	pub enable_association_engine: Option<bool>,
}
