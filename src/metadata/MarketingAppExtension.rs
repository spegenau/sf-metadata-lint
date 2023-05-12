use crate::metadata::MarketingAppExtAction::MarketingAppExtAction;
use crate::metadata::MarketingAppExtActivity::MarketingAppExtActivity;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MarketingAppExtension  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "marketingAppExtActions")]
	pub marketing_app_ext_actions: Option<Vec<MarketingAppExtAction>>,
	#[serde(rename = "marketingAppExtActivities")]
	pub marketing_app_ext_activities: Option<Vec<MarketingAppExtActivity>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
