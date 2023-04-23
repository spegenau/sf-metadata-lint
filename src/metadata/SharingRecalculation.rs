use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SharingRecalculation  {
	#[serde(rename = "className")]
	pub class_name: String,
}
