use crate::metadata::MessagingChannelActionParameterMapping::MessagingChannelActionParameterMapping;
use crate::metadata::MessagingChannelStandardParameterType::MessagingChannelStandardParameterType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MessagingChannelStandardParameter  {
	#[serde(rename = "actionParameterMappings")]
	pub action_parameter_mappings: Option<Vec<MessagingChannelActionParameterMapping>>,
	#[serde(rename = "parameterType")]
	pub parameter_type: MessagingChannelStandardParameterType,
}
