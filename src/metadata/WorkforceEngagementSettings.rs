use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WorkforceEngagementSettings  {
	#[serde(rename = "enableHistoricalAdherence")]
	pub enable_historical_adherence: Option<bool>,
	#[serde(rename = "enableIndividualAdherence")]
	pub enable_individual_adherence: Option<bool>,
	#[serde(rename = "enableIntradayManagement")]
	pub enable_intraday_management: Option<bool>,
	#[serde(rename = "enableMachineLearningForecasting")]
	pub enable_machine_learning_forecasting: Option<bool>,
	#[serde(rename = "enableRealTimeAdherence")]
	pub enable_real_time_adherence: Option<bool>,
	#[serde(rename = "enableWorkforceEngagement")]
	pub enable_workforce_engagement: Option<bool>,
	#[serde(rename = "enableWorkforceEngagementConfiguration")]
	pub enable_workforce_engagement_configuration: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
