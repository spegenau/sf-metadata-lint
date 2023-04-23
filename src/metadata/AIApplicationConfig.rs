use crate::metadata::AIScoringMode::AIScoringMode;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AIApplicationConfig  {
	#[serde(rename = "aiApplicationDeveloperName")]
	pub ai_application_developer_name: String,
	#[serde(rename = "applicationId")]
	pub application_id: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "insightReasonEnabled")]
	pub insight_reason_enabled: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "rank")]
	pub rank: Option<i32>,
	#[serde(rename = "scoringMode")]
	pub scoring_mode: Option<AIScoringMode>,
}
