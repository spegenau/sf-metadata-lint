use crate::metadata::ActionLinkHttpMethod::ActionLinkHttpMethod;
use crate::metadata::ActionLinkType::ActionLinkType;
use crate::metadata::ActionLinkUserVisibility::ActionLinkUserVisibility;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionLinkTemplate  {
	#[serde(rename = "actionUrl")]
	pub action_url: String,
	#[serde(rename = "headers")]
	pub headers: Option<String>,
	#[serde(rename = "isConfirmationRequired")]
	pub is_confirmation_required: bool,
	#[serde(rename = "isGroupDefault")]
	pub is_group_default: bool,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "labelKey")]
	pub label_key: String,
	#[serde(rename = "linkType")]
	pub link_type: ActionLinkType,
	#[serde(rename = "method")]
	pub method: ActionLinkHttpMethod,
	#[serde(rename = "position")]
	pub position: i32,
	#[serde(rename = "requestBody")]
	pub request_body: Option<String>,
	#[serde(rename = "userAlias")]
	pub user_alias: Option<String>,
	#[serde(rename = "userVisibility")]
	pub user_visibility: ActionLinkUserVisibility,
}
