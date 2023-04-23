use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatDeploymentDomainWhitelist  {
	#[serde(rename = "domain")]
	pub domain: Option<Vec<String>>,
}
