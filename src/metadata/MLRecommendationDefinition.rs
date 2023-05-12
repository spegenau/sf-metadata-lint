use crate::metadata::MLFilter::MLFilter;
use crate::metadata::MLRecommendationDefinitionStatus::MLRecommendationDefinitionStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MLRecommendationDefinition  {
	#[serde(rename = "aiApplicationDeveloperName")]
	pub ai_application_developer_name: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "externalId")]
	pub external_id: Option<String>,
	#[serde(rename = "interactionDateTimeField")]
	pub interaction_date_time_field: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "negativeExpression")]
	pub negative_expression: Option<MLFilter>,
	#[serde(rename = "positiveExpression")]
	pub positive_expression: Option<MLFilter>,
	#[serde(rename = "status")]
	pub status: MLRecommendationDefinitionStatus,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
