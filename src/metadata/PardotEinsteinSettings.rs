use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PardotEinsteinSettings  {
	#[serde(rename = "enableCampaignInsight")]
	pub enable_campaign_insight: Option<bool>,
	#[serde(rename = "enableEngagementScore")]
	pub enable_engagement_score: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
