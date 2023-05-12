use crate::metadata::AgentConfigAssignments::AgentConfigAssignments;
use crate::metadata::AgentConfigButtons::AgentConfigButtons;
use crate::metadata::AgentConfigSkills::AgentConfigSkills;
use crate::metadata::SupervisorAgentConfigSkills::SupervisorAgentConfigSkills;
use crate::metadata::SupervisorAgentStatusFilter::SupervisorAgentStatusFilter;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatAgentConfig  {
	#[serde(rename = "assignments")]
	pub assignments: Option<AgentConfigAssignments>,
	#[serde(rename = "autoGreeting")]
	pub auto_greeting: Option<String>,
	#[serde(rename = "capacity")]
	pub capacity: Option<i32>,
	#[serde(rename = "criticalWaitTime")]
	pub critical_wait_time: Option<i32>,
	#[serde(rename = "customAgentName")]
	pub custom_agent_name: Option<String>,
	#[serde(rename = "disableTransferConferenceGreeting")]
	pub disable_transfer_conference_greeting: Option<bool>,
	#[serde(rename = "enableAgentFileTransfer")]
	pub enable_agent_file_transfer: Option<bool>,
	#[serde(rename = "enableAgentSneakPeek")]
	pub enable_agent_sneak_peek: Option<bool>,
	#[serde(rename = "enableAssistanceFlag")]
	pub enable_assistance_flag: Option<bool>,
	#[serde(rename = "enableAutoAwayOnDecline")]
	pub enable_auto_away_on_decline: Option<bool>,
	#[serde(rename = "enableAutoAwayOnPushTimeout")]
	pub enable_auto_away_on_push_timeout: Option<bool>,
	#[serde(rename = "enableChatConferencing")]
	pub enable_chat_conferencing: Option<bool>,
	#[serde(rename = "enableChatMonitoring")]
	pub enable_chat_monitoring: Option<bool>,
	#[serde(rename = "enableChatTransferToAgent")]
	pub enable_chat_transfer_to_agent: Option<bool>,
	#[serde(rename = "enableChatTransferToButton")]
	pub enable_chat_transfer_to_button: Option<bool>,
	#[serde(rename = "enableChatTransferToSkill")]
	pub enable_chat_transfer_to_skill: Option<bool>,
	#[serde(rename = "enableLogoutSound")]
	pub enable_logout_sound: Option<bool>,
	#[serde(rename = "enableNotifications")]
	pub enable_notifications: Option<bool>,
	#[serde(rename = "enableRequestSound")]
	pub enable_request_sound: Option<bool>,
	#[serde(rename = "enableSneakPeek")]
	pub enable_sneak_peek: Option<bool>,
	#[serde(rename = "enableVisitorBlocking")]
	pub enable_visitor_blocking: Option<bool>,
	#[serde(rename = "enableWhisperMessage")]
	pub enable_whisper_message: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "supervisorDefaultAgentStatusFilter")]
	pub supervisor_default_agent_status_filter: Option<SupervisorAgentStatusFilter>,
	#[serde(rename = "supervisorDefaultButtonFilter")]
	pub supervisor_default_button_filter: Option<String>,
	#[serde(rename = "supervisorDefaultSkillFilter")]
	pub supervisor_default_skill_filter: Option<String>,
	#[serde(rename = "supervisorSkills")]
	pub supervisor_skills: Option<SupervisorAgentConfigSkills>,
	#[serde(rename = "transferableButtons")]
	pub transferable_buttons: Option<AgentConfigButtons>,
	#[serde(rename = "transferableSkills")]
	pub transferable_skills: Option<AgentConfigSkills>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
