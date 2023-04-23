use crate::metadata::Territory2SettingsOpportunityFilter::Territory2SettingsOpportunityFilter;
use crate::metadata::Territory2SupportedObject::Territory2SupportedObject;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory2Settings  {
	#[serde(rename = "defaultAccountAccessLevel")]
	pub default_account_access_level: Option<String>,
	#[serde(rename = "defaultCaseAccessLevel")]
	pub default_case_access_level: Option<String>,
	#[serde(rename = "defaultContactAccessLevel")]
	pub default_contact_access_level: Option<String>,
	#[serde(rename = "defaultOpportunityAccessLevel")]
	pub default_opportunity_access_level: Option<String>,
	#[serde(rename = "enableTerritoryManagement2")]
	pub enable_territory_management_2: Option<bool>,
	#[serde(rename = "opportunityFilterSettings")]
	pub opportunity_filter_settings: Option<Territory2SettingsOpportunityFilter>,
	#[serde(rename = "showTM2EnabledBanner")]
	pub show_tm_2_enabled_banner: Option<bool>,
	#[serde(rename = "supportedObjects")]
	pub supported_objects: Option<Vec<Territory2SupportedObject>>,
	#[serde(rename = "t2ForecastAccessLevel")]
	pub t_2_forecast_access_level: Option<String>,
	#[serde(rename = "tm2BypassRealignAccInsert")]
	pub tm_2_bypass_realign_acc_insert: Option<bool>,
	#[serde(rename = "tm2EnableUserAssignmentLog")]
	pub tm_2_enable_user_assignment_log: Option<bool>,
}
