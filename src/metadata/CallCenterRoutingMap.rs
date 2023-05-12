use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CallCenterRoutingMap  {
	#[serde(rename = "callCenter")]
	pub call_center: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "externalId")]
	pub external_id: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "quickConnect")]
	pub quick_connect: Option<String>,
	#[serde(rename = "referenceRecord")]
	pub reference_record: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
