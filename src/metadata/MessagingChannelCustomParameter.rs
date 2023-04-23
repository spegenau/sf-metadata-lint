use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::MessagingChannelActionParameterMapping::MessagingChannelActionParameterMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MessagingChannelCustomParameter  {
	#[serde(rename = "actionParameterMappings")]
	pub action_parameter_mappings: Option<Vec<MessagingChannelActionParameterMapping>>,
	#[serde(rename = "externalParameterName")]
	pub external_parameter_name: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "maxLength")]
	pub max_length: Option<i32>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "parameterDataType")]
	pub parameter_data_type: FlowDataType,
}
