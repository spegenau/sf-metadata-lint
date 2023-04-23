use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AudienceCriteriaValue  {
	#[serde(rename = "audienceDeveloperName")]
	pub audience_developer_name: Option<String>,
	#[serde(rename = "city")]
	pub city: Option<String>,
	#[serde(rename = "country")]
	pub country: Option<String>,
	#[serde(rename = "domain")]
	pub domain: Option<String>,
	#[serde(rename = "entityField")]
	pub entity_field: Option<String>,
	#[serde(rename = "entityType")]
	pub entity_type: Option<String>,
	#[serde(rename = "fieldValue")]
	pub field_value: Option<String>,
	#[serde(rename = "isEnabled")]
	pub is_enabled: Option<String>,
	#[serde(rename = "permissionName")]
	pub permission_name: Option<String>,
	#[serde(rename = "permissionType")]
	pub permission_type: Option<String>,
	#[serde(rename = "profile")]
	pub profile: Option<String>,
	#[serde(rename = "subdivision")]
	pub subdivision: Option<String>,
}
