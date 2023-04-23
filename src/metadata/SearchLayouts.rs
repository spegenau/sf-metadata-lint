use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SearchLayouts  {
	#[serde(rename = "customTabListAdditionalFields")]
	pub custom_tab_list_additional_fields: Option<Vec<String>>,
	#[serde(rename = "excludedStandardButtons")]
	pub excluded_standard_buttons: Option<Vec<String>>,
	#[serde(rename = "listViewButtons")]
	pub list_view_buttons: Option<Vec<String>>,
	#[serde(rename = "lookupDialogsAdditionalFields")]
	pub lookup_dialogs_additional_fields: Option<Vec<String>>,
	#[serde(rename = "lookupFilterFields")]
	pub lookup_filter_fields: Option<Vec<String>>,
	#[serde(rename = "lookupPhoneDialogsAdditionalFields")]
	pub lookup_phone_dialogs_additional_fields: Option<Vec<String>>,
	#[serde(rename = "massQuickActions")]
	pub mass_quick_actions: Option<Vec<String>>,
	#[serde(rename = "searchFilterFields")]
	pub search_filter_fields: Option<Vec<String>>,
	#[serde(rename = "searchResultsAdditionalFields")]
	pub search_results_additional_fields: Option<Vec<String>>,
	#[serde(rename = "searchResultsCustomButtons")]
	pub search_results_custom_buttons: Option<Vec<String>>,
}
