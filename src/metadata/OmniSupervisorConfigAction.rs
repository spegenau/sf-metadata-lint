use crate::metadata::OmniSupervisorActionName::OmniSupervisorActionName;
use crate::metadata::OmniSupervisorActionTab::OmniSupervisorActionTab;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfigAction  {
	#[serde(rename = "actionName")]
	pub action_name: OmniSupervisorActionName,
	#[serde(rename = "actionTab")]
	pub action_tab: OmniSupervisorActionTab,
	#[serde(rename = "customActionFlow")]
	pub custom_action_flow: Option<String>,
	#[serde(rename = "displayOrder")]
	pub display_order: i32,
}
