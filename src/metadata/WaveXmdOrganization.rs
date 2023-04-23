use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdOrganization  {
	#[serde(rename = "instanceUrl")]
	pub instance_url: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "organizationIdentifier")]
	pub organization_identifier: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
