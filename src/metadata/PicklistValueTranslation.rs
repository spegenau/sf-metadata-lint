use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PicklistValueTranslation  {
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "translation")]
	pub translation: Option<String>,
}
