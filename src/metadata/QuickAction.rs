use crate::metadata::ActionSubtype::ActionSubtype;
use crate::metadata::FieldOverride::FieldOverride;
use crate::metadata::QuickActionLabel::QuickActionLabel;
use crate::metadata::QuickActionLayout::QuickActionLayout;
use crate::metadata::QuickActionSendEmailOptions::QuickActionSendEmailOptions;
use crate::metadata::QuickActionType::QuickActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuickAction  {
	#[serde(rename = "actionSubtype")]
	pub action_subtype: Option<ActionSubtype>,
	#[serde(rename = "canvas")]
	pub canvas: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "fieldOverrides")]
	pub field_overrides: Option<Vec<FieldOverride>>,
	#[serde(rename = "flowDefinition")]
	pub flow_definition: Option<String>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "lightningComponent")]
	pub lightning_component: Option<String>,
	#[serde(rename = "lightningWebComponent")]
	pub lightning_web_component: Option<String>,
	#[serde(rename = "optionsCreateFeedItem")]
	pub options_create_feed_item: bool,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "quickActionLayout")]
	pub quick_action_layout: Option<QuickActionLayout>,
	#[serde(rename = "quickActionSendEmailOptions")]
	pub quick_action_send_email_options: Option<QuickActionSendEmailOptions>,
	#[serde(rename = "standardLabel")]
	pub standard_label: Option<QuickActionLabel>,
	#[serde(rename = "successMessage")]
	pub success_message: Option<String>,
	#[serde(rename = "targetObject")]
	pub target_object: Option<String>,
	#[serde(rename = "targetParentField")]
	pub target_parent_field: Option<String>,
	#[serde(rename = "targetRecordType")]
	pub target_record_type: Option<String>,
	#[serde(rename = "type")]
	pub _type: QuickActionType,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
