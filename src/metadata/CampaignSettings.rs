use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CampaignSettings  {
	#[serde(rename = "aiAttributionTimeframe")]
	pub ai_attribution_timeframe: Option<i32>,
	#[serde(rename = "enableAIAttribution")]
	pub enable_ai_attribution: Option<bool>,
	#[serde(rename = "enableAccountsAsCM")]
	pub enable_accounts_as_cm: Option<bool>,
	#[serde(rename = "enableAutoCampInfluenceDisabled")]
	pub enable_auto_camp_influence_disabled: Option<bool>,
	#[serde(rename = "enableB2bmaCampaignInfluence2")]
	pub enable_b_2_bma_campaign_influence_2: Option<bool>,
	#[serde(rename = "enableCampaignHistoryTrackEnabled")]
	pub enable_campaign_history_track_enabled: Option<bool>,
	#[serde(rename = "enableCampaignInfluence2")]
	pub enable_campaign_influence_2: Option<bool>,
	#[serde(rename = "enableCampaignMemberTWCF")]
	pub enable_campaign_member_twcf: Option<bool>,
	#[serde(rename = "enableEKAI")]
	pub enable_ekai: Option<bool>,
	#[serde(rename = "enableSuppressNoValueCI2")]
	pub enable_suppress_no_value_ci_2: Option<bool>,
}
