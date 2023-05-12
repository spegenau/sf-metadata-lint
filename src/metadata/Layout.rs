use crate::metadata::CustomConsoleComponents::CustomConsoleComponents;
use crate::metadata::FeedLayout::FeedLayout;
use crate::metadata::LayoutHeader::LayoutHeader;
use crate::metadata::LayoutSection::LayoutSection;
use crate::metadata::MiniLayout::MiniLayout;
use crate::metadata::PlatformActionList::PlatformActionList;
use crate::metadata::QuickActionList::QuickActionList;
use crate::metadata::RelatedContent::RelatedContent;
use crate::metadata::RelatedListItem::RelatedListItem;
use crate::metadata::SummaryLayout::SummaryLayout;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Layout  {
	#[serde(rename = "customButtons")]
	pub custom_buttons: Option<Vec<String>>,
	#[serde(rename = "customConsoleComponents")]
	pub custom_console_components: Option<CustomConsoleComponents>,
	#[serde(rename = "emailDefault")]
	pub email_default: Option<bool>,
	#[serde(rename = "excludeButtons")]
	pub exclude_buttons: Option<Vec<String>>,
	#[serde(rename = "feedLayout")]
	pub feed_layout: Option<FeedLayout>,
	#[serde(rename = "headers")]
	pub headers: Option<Vec<LayoutHeader>>,
	#[serde(rename = "layoutSections")]
	pub layout_sections: Option<Vec<LayoutSection>>,
	#[serde(rename = "miniLayout")]
	pub mini_layout: Option<MiniLayout>,
	#[serde(rename = "multilineLayoutFields")]
	pub multiline_layout_fields: Option<Vec<String>>,
	#[serde(rename = "platformActionList")]
	pub platform_action_list: Option<PlatformActionList>,
	#[serde(rename = "quickActionList")]
	pub quick_action_list: Option<QuickActionList>,
	#[serde(rename = "relatedContent")]
	pub related_content: Option<RelatedContent>,
	#[serde(rename = "relatedLists")]
	pub related_lists: Option<Vec<RelatedListItem>>,
	#[serde(rename = "relatedObjects")]
	pub related_objects: Option<Vec<String>>,
	#[serde(rename = "runAssignmentRulesDefault")]
	pub run_assignment_rules_default: Option<bool>,
	#[serde(rename = "showEmailCheckbox")]
	pub show_email_checkbox: Option<bool>,
	#[serde(rename = "showHighlightsPanel")]
	pub show_highlights_panel: Option<bool>,
	#[serde(rename = "showInteractionLogPanel")]
	pub show_interaction_log_panel: Option<bool>,
	#[serde(rename = "showKnowledgeComponent")]
	pub show_knowledge_component: Option<bool>,
	#[serde(rename = "showRunAssignmentRulesCheckbox")]
	pub show_run_assignment_rules_checkbox: Option<bool>,
	#[serde(rename = "showSolutionSection")]
	pub show_solution_section: Option<bool>,
	#[serde(rename = "showSubmitAndAttachButton")]
	pub show_submit_and_attach_button: Option<bool>,
	#[serde(rename = "summaryLayout")]
	pub summary_layout: Option<SummaryLayout>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
