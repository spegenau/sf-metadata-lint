use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OauthCustomScopeApp  {
	#[serde(rename = "connectedApp")]
	pub connected_app: String,
}
