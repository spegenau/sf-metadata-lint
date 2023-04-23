use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EntitlementTemplate  {
	#[serde(rename = "businessHours")]
	pub business_hours: Option<String>,
	#[serde(rename = "casesPerEntitlement")]
	pub cases_per_entitlement: Option<i32>,
	#[serde(rename = "entitlementProcess")]
	pub entitlement_process: Option<String>,
	#[serde(rename = "isPerIncident")]
	pub is_per_incident: Option<bool>,
	#[serde(rename = "term")]
	pub term: Option<i32>,
	#[serde(rename = "type")]
	pub _type: Option<String>,
}
