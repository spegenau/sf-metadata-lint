use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RoleOrTerritory  {
	#[serde(rename = "caseAccessLevel")]
	pub case_access_level: Option<String>,
	#[serde(rename = "contactAccessLevel")]
	pub contact_access_level: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "mayForecastManagerShare")]
	pub may_forecast_manager_share: Option<bool>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "opportunityAccessLevel")]
	pub opportunity_access_level: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
