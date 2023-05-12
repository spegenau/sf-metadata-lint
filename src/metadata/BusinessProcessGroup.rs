use crate::metadata::BusinessProcessDefinition::BusinessProcessDefinition;
use crate::metadata::SurveyQuestionType::SurveyQuestionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BusinessProcessGroup  {
	#[serde(rename = "businessProcessDefinitions")]
	pub business_process_definitions: Option<Vec<BusinessProcessDefinition>>,
	#[serde(rename = "customerSatisfactionMetric")]
	pub customer_satisfaction_metric: SurveyQuestionType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
