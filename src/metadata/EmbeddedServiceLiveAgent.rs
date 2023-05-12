use crate::metadata::EmbeddedServiceFontSize::EmbeddedServiceFontSize;
use crate::metadata::EmbeddedServiceQuickAction::EmbeddedServiceQuickAction;
use crate::metadata::EmbeddedServiceScenario::EmbeddedServiceScenario;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmbeddedServiceLiveAgent  {
	#[serde(rename = "avatarImg")]
	pub avatar_img: Option<String>,
	#[serde(rename = "embeddedServiceConfig")]
	pub embedded_service_config: String,
	#[serde(rename = "embeddedServiceQuickActions")]
	pub embedded_service_quick_actions: Option<Vec<EmbeddedServiceQuickAction>>,
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "fontSize")]
	pub font_size: EmbeddedServiceFontSize,
	#[serde(rename = "isOfflineCaseEnabled")]
	pub is_offline_case_enabled: Option<bool>,
	#[serde(rename = "isQueuePositionEnabled")]
	pub is_queue_position_enabled: Option<bool>,
	#[serde(rename = "liveAgentChatUrl")]
	pub live_agent_chat_url: Option<String>,
	#[serde(rename = "liveAgentContentUrl")]
	pub live_agent_content_url: Option<String>,
	#[serde(rename = "liveChatButton")]
	pub live_chat_button: String,
	#[serde(rename = "liveChatDeployment")]
	pub live_chat_deployment: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "offlineCaseBackgroundImg")]
	pub offline_case_background_img: Option<String>,
	#[serde(rename = "prechatBackgroundImg")]
	pub prechat_background_img: Option<String>,
	#[serde(rename = "prechatEnabled")]
	pub prechat_enabled: bool,
	#[serde(rename = "prechatJson")]
	pub prechat_json: Option<String>,
	#[serde(rename = "scenario")]
	pub scenario: EmbeddedServiceScenario,
	#[serde(rename = "smallCompanyLogoImg")]
	pub small_company_logo_img: Option<String>,
	#[serde(rename = "waitingStateBackgroundImg")]
	pub waiting_state_background_img: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
