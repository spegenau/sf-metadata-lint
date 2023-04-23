use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MarketingAppExtAction  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "actionParams")]
	pub action_params: Option<String>,
	#[serde(rename = "actionSchema")]
	pub action_schema: Option<String>,
	#[serde(rename = "actionSelector")]
	pub action_selector: String,
	#[serde(rename = "apiName")]
	pub api_name: String,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "marketingAppExtension")]
	pub marketing_app_extension: String,
}
