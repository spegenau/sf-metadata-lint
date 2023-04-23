use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LiveAgentConfig  {
	#[serde(rename = "enableLiveChat")]
	pub enable_live_chat: Option<bool>,
	#[serde(rename = "openNewAccountSubtab")]
	pub open_new_account_subtab: Option<bool>,
	#[serde(rename = "openNewCaseSubtab")]
	pub open_new_case_subtab: Option<bool>,
	#[serde(rename = "openNewContactSubtab")]
	pub open_new_contact_subtab: Option<bool>,
	#[serde(rename = "openNewLeadSubtab")]
	pub open_new_lead_subtab: Option<bool>,
	#[serde(rename = "openNewVFPageSubtab")]
	pub open_new_vf_page_subtab: Option<bool>,
	#[serde(rename = "pageNamesToOpen")]
	pub page_names_to_open: Option<Vec<String>>,
	#[serde(rename = "showKnowledgeArticles")]
	pub show_knowledge_articles: Option<bool>,
}
