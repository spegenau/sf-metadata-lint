use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MessagingChannelActionParameterMapping  {
	#[serde(rename = "actionParameterName")]
	pub action_parameter_name: String,
}
