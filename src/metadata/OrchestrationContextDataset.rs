use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrchestrationContextDataset  {
	#[serde(rename = "datasetType")]
	pub dataset_type: String,
	#[serde(rename = "orchestrationDataset")]
	pub orchestration_dataset: String,
}
