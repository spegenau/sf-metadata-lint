use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PardotSettings  {
	#[serde(rename = "enableAIEinsteinEngageFreq")]
	pub enable_ai_einstein_engage_freq: Option<bool>,
	#[serde(rename = "enableAIOptimizedSendTime")]
	pub enable_ai_optimized_send_time: Option<bool>,
	#[serde(rename = "enableB2bmaAppEnabled")]
	pub enable_b_2_bma_app_enabled: Option<bool>,
	#[serde(rename = "enableEngagementHistoryDashboards")]
	pub enable_engagement_history_dashboards: Option<bool>,
	#[serde(rename = "enableEnhancedProspectCustomFieldsSync")]
	pub enable_enhanced_prospect_custom_fields_sync: Option<bool>,
	#[serde(rename = "enablePardotAppV1Enabled")]
	pub enable_pardot_app_v_1_enabled: Option<bool>,
	#[serde(rename = "enablePardotEnabled")]
	pub enable_pardot_enabled: Option<bool>,
	#[serde(rename = "enablePardotObjectSync")]
	pub enable_pardot_object_sync: Option<bool>,
	#[serde(rename = "enableProspectActivityDataset")]
	pub enable_prospect_activity_dataset: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
