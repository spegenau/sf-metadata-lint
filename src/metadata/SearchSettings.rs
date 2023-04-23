use crate::metadata::SearchSettingsByObject::SearchSettingsByObject;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SearchSettings  {
	#[serde(rename = "documentContentSearchEnabled")]
	pub document_content_search_enabled: bool,
	#[serde(rename = "enableAdvancedSearchInAlohaSidebar")]
	pub enable_advanced_search_in_aloha_sidebar: Option<bool>,
	#[serde(rename = "enableEinsteinSearchAssistantDialog")]
	pub enable_einstein_search_assistant_dialog: Option<bool>,
	#[serde(rename = "enableEinsteinSearchEs4kPilot")]
	pub enable_einstein_search_es_4_k_pilot: Option<bool>,
	#[serde(rename = "enableEinsteinSearchNLSFilters")]
	pub enable_einstein_search_nls_filters: Option<bool>,
	#[serde(rename = "enableEinsteinSearchNaturalLanguage")]
	pub enable_einstein_search_natural_language: Option<bool>,
	#[serde(rename = "enableEinsteinSearchPersonalization")]
	pub enable_einstein_search_personalization: Option<bool>,
	#[serde(rename = "enableEinsteinSearchQA")]
	pub enable_einstein_search_qa: Option<bool>,
	#[serde(rename = "enablePersonalTagging")]
	pub enable_personal_tagging: Option<bool>,
	#[serde(rename = "enablePublicTagging")]
	pub enable_public_tagging: Option<bool>,
	#[serde(rename = "enableQuerySuggestionPigOn")]
	pub enable_query_suggestion_pig_on: Option<bool>,
	#[serde(rename = "enableSalesforceGeneratedSynonyms")]
	pub enable_salesforce_generated_synonyms: Option<bool>,
	#[serde(rename = "enableSearchTermHistory")]
	pub enable_search_term_history: Option<bool>,
	#[serde(rename = "enableSetupSearch")]
	pub enable_setup_search: Option<bool>,
	#[serde(rename = "enableSuggestArticlesLinksOnly")]
	pub enable_suggest_articles_links_only: Option<bool>,
	#[serde(rename = "enableUseDefaultSearchEntity")]
	pub enable_use_default_search_entity: Option<bool>,
	#[serde(rename = "optimizeSearchForCJKEnabled")]
	pub optimize_search_for_cjk_enabled: bool,
	#[serde(rename = "recentlyViewedUsersForBlankLookupEnabled")]
	pub recently_viewed_users_for_blank_lookup_enabled: bool,
	#[serde(rename = "searchSettingsByObject")]
	pub search_settings_by_object: SearchSettingsByObject,
	#[serde(rename = "sidebarAutoCompleteEnabled")]
	pub sidebar_auto_complete_enabled: bool,
	#[serde(rename = "sidebarDropDownListEnabled")]
	pub sidebar_drop_down_list_enabled: bool,
	#[serde(rename = "sidebarLimitToItemsIOwnCheckboxEnabled")]
	pub sidebar_limit_to_items_i_own_checkbox_enabled: bool,
	#[serde(rename = "singleSearchResultShortcutEnabled")]
	pub single_search_result_shortcut_enabled: bool,
	#[serde(rename = "spellCorrectKnowledgeSearchEnabled")]
	pub spell_correct_knowledge_search_enabled: bool,
}
