use crate::metadata::OrchestrationContextDataset::OrchestrationContextDataset;
use crate::metadata::OrchestrationContextEvent::OrchestrationContextEvent;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OrchestrationContext  {
	#[serde(rename = "datasets")]
	pub datasets: Option<Vec<OrchestrationContextDataset>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "events")]
	pub events: Option<Vec<OrchestrationContextEvent>>,
	#[serde(rename = "imageFile")]
	pub image_file: String,
	#[serde(rename = "imageScale")]
	pub image_scale: i32,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "runtimeType")]
	pub runtime_type: String,
	#[serde(rename = "salesforceObject")]
	pub salesforce_object: Option<String>,
	#[serde(rename = "salesforceObjectPrimaryKey")]
	pub salesforce_object_primary_key: Option<String>,
}
