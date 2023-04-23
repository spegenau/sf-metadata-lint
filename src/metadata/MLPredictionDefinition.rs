use crate::metadata::AIPredictionType::AIPredictionType;
use crate::metadata::MLFilter::MLFilter;
use crate::metadata::MLPredictionDefinitionStatus::MLPredictionDefinitionStatus;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MLPredictionDefinition  {
	#[serde(rename = "aiApplicationDeveloperName")]
	pub ai_application_developer_name: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: Option<String>,
	#[serde(rename = "negativeExpression")]
	pub negative_expression: Option<MLFilter>,
	#[serde(rename = "positiveExpression")]
	pub positive_expression: Option<MLFilter>,
	#[serde(rename = "predictionField")]
	pub prediction_field: Option<String>,
	#[serde(rename = "priority")]
	pub priority: Option<i32>,
	#[serde(rename = "pushbackField")]
	pub pushback_field: Option<String>,
	#[serde(rename = "status")]
	pub status: MLPredictionDefinitionStatus,
	#[serde(rename = "type")]
	pub _type: AIPredictionType,
}
