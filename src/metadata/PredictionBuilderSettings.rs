use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PredictionBuilderSettings  {
	#[serde(rename = "enablePredictionBuilder")]
	pub enable_prediction_builder: Option<bool>,
	#[serde(rename = "isPredictionBuilderStarted")]
	pub is_prediction_builder_started: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
