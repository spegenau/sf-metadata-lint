use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProfileLayoutAssignment  {
	#[serde(rename = "layout")]
	pub layout: String,
	#[serde(rename = "recordType")]
	pub record_type: Option<String>,
}
