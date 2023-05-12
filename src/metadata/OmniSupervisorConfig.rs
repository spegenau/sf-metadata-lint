use crate::metadata::OmniSuperSkillVisibilityType::OmniSuperSkillVisibilityType;
use crate::metadata::OmniSupervisorConfigAction::OmniSupervisorConfigAction;
use crate::metadata::OmniSupervisorConfigGroup::OmniSupervisorConfigGroup;
use crate::metadata::OmniSupervisorConfigProfile::OmniSupervisorConfigProfile;
use crate::metadata::OmniSupervisorConfigQueue::OmniSupervisorConfigQueue;
use crate::metadata::OmniSupervisorConfigSkill::OmniSupervisorConfigSkill;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniSupervisorConfig  {
	#[serde(rename = "isTimelineHidden")]
	pub is_timeline_hidden: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "omniSupervisorConfigAction")]
	pub omni_supervisor_config_action: Option<Vec<OmniSupervisorConfigAction>>,
	#[serde(rename = "omniSupervisorConfigGroup")]
	pub omni_supervisor_config_group: Option<Vec<OmniSupervisorConfigGroup>>,
	#[serde(rename = "omniSupervisorConfigProfile")]
	pub omni_supervisor_config_profile: Option<Vec<OmniSupervisorConfigProfile>>,
	#[serde(rename = "omniSupervisorConfigQueue")]
	pub omni_supervisor_config_queue: Option<Vec<OmniSupervisorConfigQueue>>,
	#[serde(rename = "omniSupervisorConfigSkill")]
	pub omni_supervisor_config_skill: Option<Vec<OmniSupervisorConfigSkill>>,
	#[serde(rename = "skillVisibility")]
	pub skill_visibility: Option<OmniSuperSkillVisibilityType>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
