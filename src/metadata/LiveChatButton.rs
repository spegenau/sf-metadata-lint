use crate::metadata::Language::Language;
use crate::metadata::LiveChatButtonDeployments::LiveChatButtonDeployments;
use crate::metadata::LiveChatButtonInviteEndPosition::LiveChatButtonInviteEndPosition;
use crate::metadata::LiveChatButtonInviteStartPosition::LiveChatButtonInviteStartPosition;
use crate::metadata::LiveChatButtonPresentation::LiveChatButtonPresentation;
use crate::metadata::LiveChatButtonRoutingType::LiveChatButtonRoutingType;
use crate::metadata::LiveChatButtonSkills::LiveChatButtonSkills;
use crate::metadata::LiveChatButtonType::LiveChatButtonType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveChatButton  {
	#[serde(rename = "animation")]
	pub animation: Option<LiveChatButtonPresentation>,
	#[serde(rename = "autoGreeting")]
	pub auto_greeting: Option<String>,
	#[serde(rename = "chasitorIdleTimeout")]
	pub chasitor_idle_timeout: Option<i32>,
	#[serde(rename = "chasitorIdleTimeoutWarning")]
	pub chasitor_idle_timeout_warning: Option<i32>,
	#[serde(rename = "chatPage")]
	pub chat_page: Option<String>,
	#[serde(rename = "customAgentName")]
	pub custom_agent_name: Option<String>,
	#[serde(rename = "deployments")]
	pub deployments: Option<LiveChatButtonDeployments>,
	#[serde(rename = "enableQueue")]
	pub enable_queue: Option<bool>,
	#[serde(rename = "inviteEndPosition")]
	pub invite_end_position: Option<LiveChatButtonInviteEndPosition>,
	#[serde(rename = "inviteImage")]
	pub invite_image: Option<String>,
	#[serde(rename = "inviteStartPosition")]
	pub invite_start_position: Option<LiveChatButtonInviteStartPosition>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "numberOfReroutingAttempts")]
	pub number_of_rerouting_attempts: Option<i32>,
	#[serde(rename = "offlineImage")]
	pub offline_image: Option<String>,
	#[serde(rename = "onlineImage")]
	pub online_image: Option<String>,
	#[serde(rename = "optionsCustomRoutingIsEnabled")]
	pub options_custom_routing_is_enabled: Option<bool>,
	#[serde(rename = "optionsHasChasitorIdleTimeout")]
	pub options_has_chasitor_idle_timeout: bool,
	#[serde(rename = "optionsHasInviteAfterAccept")]
	pub options_has_invite_after_accept: Option<bool>,
	#[serde(rename = "optionsHasInviteAfterReject")]
	pub options_has_invite_after_reject: Option<bool>,
	#[serde(rename = "optionsHasRerouteDeclinedRequest")]
	pub options_has_reroute_declined_request: Option<bool>,
	#[serde(rename = "optionsIsAutoAccept")]
	pub options_is_auto_accept: Option<bool>,
	#[serde(rename = "optionsIsInviteAutoRemove")]
	pub options_is_invite_auto_remove: Option<bool>,
	#[serde(rename = "overallQueueLength")]
	pub overall_queue_length: Option<i32>,
	#[serde(rename = "perAgentQueueLength")]
	pub per_agent_queue_length: Option<i32>,
	#[serde(rename = "postChatPage")]
	pub post_chat_page: Option<String>,
	#[serde(rename = "postChatUrl")]
	pub post_chat_url: Option<String>,
	#[serde(rename = "preChatFormPage")]
	pub pre_chat_form_page: Option<String>,
	#[serde(rename = "preChatFormUrl")]
	pub pre_chat_form_url: Option<String>,
	#[serde(rename = "pushTimeOut")]
	pub push_time_out: Option<i32>,
	#[serde(rename = "routingType")]
	pub routing_type: LiveChatButtonRoutingType,
	#[serde(rename = "site")]
	pub site: Option<String>,
	#[serde(rename = "skills")]
	pub skills: Option<LiveChatButtonSkills>,
	#[serde(rename = "timeToRemoveInvite")]
	pub time_to_remove_invite: Option<i32>,
	#[serde(rename = "type")]
	pub _type: LiveChatButtonType,
	#[serde(rename = "windowLanguage")]
	pub window_language: Option<Language>,
}
