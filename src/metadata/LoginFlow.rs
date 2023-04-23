use crate::metadata::LoginFlowType::LoginFlowType;
use crate::metadata::UiLoginFlowType::UiLoginFlowType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LoginFlow  {
	#[serde(rename = "flow")]
	pub flow: Option<String>,
	#[serde(rename = "flowType")]
	pub flow_type: LoginFlowType,
	#[serde(rename = "friendlyName")]
	pub friendly_name: String,
	#[serde(rename = "uiLoginFlowType")]
	pub ui_login_flow_type: UiLoginFlowType,
	#[serde(rename = "useLightningRuntime")]
	pub use_lightning_runtime: Option<bool>,
	#[serde(rename = "vfFlowPage")]
	pub vf_flow_page: Option<String>,
	#[serde(rename = "vfFlowPageTitle")]
	pub vf_flow_page_title: Option<String>,
}
