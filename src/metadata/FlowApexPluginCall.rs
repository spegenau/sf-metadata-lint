use crate::metadata::FlowApexPluginCallInputParameter::FlowApexPluginCallInputParameter;
use crate::metadata::FlowApexPluginCallOutputParameter::FlowApexPluginCallOutputParameter;
use crate::metadata::FlowConnector::FlowConnector;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowApexPluginCall  {
	#[serde(rename = "apexClass")]
	pub apex_class: String,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowApexPluginCallInputParameter>>,
	#[serde(rename = "outputParameters")]
	pub output_parameters: Option<Vec<FlowApexPluginCallOutputParameter>>,
}
