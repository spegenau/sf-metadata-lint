use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AuraDefinition  {
	#[serde(rename = "defType")]
	pub def_type: String,
	#[serde(rename = "source")]
	pub source: String,
}
