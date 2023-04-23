use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatButtonDeployments  {
	#[serde(rename = "deployment")]
	pub deployment: Option<Vec<String>>,
}
