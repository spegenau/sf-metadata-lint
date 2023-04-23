use crate::metadata::MessagingAutoResponseType::MessagingAutoResponseType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MessagingAutoResponse  {
	#[serde(rename = "response")]
	pub response: String,
	#[serde(rename = "type")]
	pub _type: MessagingAutoResponseType,
}
