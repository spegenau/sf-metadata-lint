use crate::metadata::ActionForNoRecordFound::ActionForNoRecordFound;
use crate::metadata::ActionForSingleRecordFound::ActionForSingleRecordFound;
use crate::metadata::ChannelType::ChannelType;
use crate::metadata::ObjectToLink::ObjectToLink;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ChannelObjectLinkingRule  {
	#[serde(rename = "actionForNoRecordFound")]
	pub action_for_no_record_found: ActionForNoRecordFound,
	#[serde(rename = "actionForSingleRecordFound")]
	pub action_for_single_record_found: ActionForSingleRecordFound,
	#[serde(rename = "channelType")]
	pub channel_type: ChannelType,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isLinkedRecordOpenedAsSubTab")]
	pub is_linked_record_opened_as_sub_tab: bool,
	#[serde(rename = "isRuleActive")]
	pub is_rule_active: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "objectToLink")]
	pub object_to_link: ObjectToLink,
	#[serde(rename = "ruleName")]
	pub rule_name: String,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
