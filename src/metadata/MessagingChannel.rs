use crate::metadata::MessagingAutoResponse::MessagingAutoResponse;
use crate::metadata::MessagingChannelCustomParameter::MessagingChannelCustomParameter;
use crate::metadata::MessagingChannelStandardParameter::MessagingChannelStandardParameter;
use crate::metadata::MessagingChannelType::MessagingChannelType;
use crate::metadata::MessagingSessionHandlerType::MessagingSessionHandlerType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MessagingChannel  {
	#[serde(rename = "automatedResponses")]
	pub automated_responses: Option<Vec<MessagingAutoResponse>>,
	#[serde(rename = "customParameters")]
	pub custom_parameters: Option<Vec<MessagingChannelCustomParameter>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "messagingChannelType")]
	pub messaging_channel_type: MessagingChannelType,
	#[serde(rename = "sessionHandlerFlow")]
	pub session_handler_flow: Option<String>,
	#[serde(rename = "sessionHandlerQueue")]
	pub session_handler_queue: String,
	#[serde(rename = "sessionHandlerType")]
	pub session_handler_type: MessagingSessionHandlerType,
	#[serde(rename = "standardParameters")]
	pub standard_parameters: Option<Vec<MessagingChannelStandardParameter>>,
}
