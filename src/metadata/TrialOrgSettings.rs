use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TrialOrgSettings  {
	#[serde(rename = "enableSampleDataDeleted")]
	pub enable_sample_data_deleted: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
