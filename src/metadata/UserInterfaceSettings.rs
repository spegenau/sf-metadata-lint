use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserInterfaceSettings  {
	#[serde(rename = "alternateAlohaListView")]
	pub alternate_aloha_list_view: Option<bool>,
	#[serde(rename = "dynamicMruActionsOff")]
	pub dynamic_mru_actions_off: Option<bool>,
	#[serde(rename = "enableAsyncRelatedLists")]
	pub enable_async_related_lists: Option<bool>,
	#[serde(rename = "enableClickjackUserPageHeaderless")]
	pub enable_clickjack_user_page_headerless: Option<bool>,
	#[serde(rename = "enableCollapsibleSections")]
	pub enable_collapsible_sections: Option<bool>,
	#[serde(rename = "enableCollapsibleSideBar")]
	pub enable_collapsible_side_bar: Option<bool>,
	#[serde(rename = "enableCustomObjectTruncate")]
	pub enable_custom_object_truncate: Option<bool>,
	#[serde(rename = "enableCustomeSideBarOnAllPages")]
	pub enable_custome_side_bar_on_all_pages: Option<bool>,
	#[serde(rename = "enableDeleteFieldHistory")]
	pub enable_delete_field_history: Option<bool>,
	#[serde(rename = "enableExternalObjectAsyncRelatedLists")]
	pub enable_external_object_async_related_lists: Option<bool>,
	#[serde(rename = "enableHoverDetails")]
	pub enable_hover_details: Option<bool>,
	#[serde(rename = "enableInlineEdit")]
	pub enable_inline_edit: Option<bool>,
	#[serde(rename = "enableNewPageLayoutEditor")]
	pub enable_new_page_layout_editor: Option<bool>,
	#[serde(rename = "enablePersonalCanvas")]
	pub enable_personal_canvas: Option<bool>,
	#[serde(rename = "enablePrintableListViews")]
	pub enable_printable_list_views: Option<bool>,
	#[serde(rename = "enableProfileCustomTabsets")]
	pub enable_profile_custom_tabsets: Option<bool>,
	#[serde(rename = "enableQuickCreate")]
	pub enable_quick_create: Option<bool>,
	#[serde(rename = "enableRelatedListHovers")]
	pub enable_related_list_hovers: Option<bool>,
	#[serde(rename = "enableTabOrganizer")]
	pub enable_tab_organizer: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
