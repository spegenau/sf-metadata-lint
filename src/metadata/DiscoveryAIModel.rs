use crate::metadata::DiscoveryAIModelStatus::DiscoveryAIModelStatus;
use crate::metadata::DiscoveryAlgorithmType::DiscoveryAlgorithmType;
use crate::metadata::DiscoveryModelField::DiscoveryModelField;
use crate::metadata::DiscoveryModelRuntimeType::DiscoveryModelRuntimeType;
use crate::metadata::DiscoveryModelSourceType::DiscoveryModelSourceType;
use crate::metadata::DiscoveryModelTransform::DiscoveryModelTransform;
use crate::metadata::DiscoveryPredictionType::DiscoveryPredictionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DiscoveryAIModel  {
	#[serde(rename = "algorithmType")]
	pub algorithm_type: DiscoveryAlgorithmType,
	#[serde(rename = "classificationThreshold")]
	pub classification_threshold: Option<f32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "modelFields")]
	pub model_fields: Option<Vec<DiscoveryModelField>>,
	#[serde(rename = "modelRuntimeType")]
	pub model_runtime_type: DiscoveryModelRuntimeType,
	#[serde(rename = "predictedField")]
	pub predicted_field: String,
	#[serde(rename = "predictionType")]
	pub prediction_type: DiscoveryPredictionType,
	#[serde(rename = "sourceType")]
	pub source_type: DiscoveryModelSourceType,
	#[serde(rename = "status")]
	pub status: DiscoveryAIModelStatus,
	#[serde(rename = "trainingMetrics")]
	pub training_metrics: Option<String>,
	#[serde(rename = "transformations")]
	pub transformations: Option<Vec<DiscoveryModelTransform>>,
}
