use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ValueTranslation  {
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "translation")]
	pub translation: Option<String>,
}
