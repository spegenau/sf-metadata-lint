use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SearchCustomization  {
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
}
