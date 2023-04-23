use crate::metadata::AudienceCriteriaType::AudienceCriteriaType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationAudienceDetail  {
	#[serde(rename = "audienceCriteriaType")]
	pub audience_criteria_type: Option<AudienceCriteriaType>,
	#[serde(rename = "audienceCriteriaValue")]
	pub audience_criteria_value: Option<String>,
	#[serde(rename = "setupName")]
	pub setup_name: Option<String>,
}
