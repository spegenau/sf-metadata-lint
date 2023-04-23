use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ProductSpecificationTypeDefinition  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "recordType")]
	pub record_type: String,
}
