use crate::metadata::AuraDefinition::AuraDefinition;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuraDefinitions  {
	#[serde(rename = "auraDefinition")]
	pub aura_definition: Option<Vec<AuraDefinition>>,
}
