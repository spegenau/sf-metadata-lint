use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StrategyNodeAiLoad  {
	#[serde(rename = "acceptanceLabel")]
	pub acceptance_label: String,
	#[serde(rename = "actionReference")]
	pub action_reference: String,
	#[serde(rename = "descriptionField")]
	pub description_field: String,
	#[serde(rename = "recommendationDefinitionDevName")]
	pub recommendation_definition_dev_name: String,
	#[serde(rename = "rejectionLabel")]
	pub rejection_label: Option<String>,
	#[serde(rename = "titleField")]
	pub title_field: String,
}
