use crate::metadata::PromptDisplayPosition::PromptDisplayPosition;
use crate::metadata::PromptDisplayType::PromptDisplayType;
use crate::metadata::PromptElementRelativePosition::PromptElementRelativePosition;
use crate::metadata::PromptImageLocation::PromptImageLocation;
use crate::metadata::PromptThemeColor::PromptThemeColor;
use crate::metadata::PromptThemeSaturation::PromptThemeSaturation;
use crate::metadata::PromptUserAccess::PromptUserAccess;
use crate::metadata::PromptUserProfileAccess::PromptUserProfileAccess;
use crate::metadata::UiFormulaRule::UiFormulaRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PromptVersion  {
	#[serde(rename = "actionButtonLabel")]
	pub action_button_label: Option<String>,
	#[serde(rename = "actionButtonLink")]
	pub action_button_link: Option<String>,
	#[serde(rename = "body")]
	pub body: String,
	#[serde(rename = "customApplication")]
	pub custom_application: Option<String>,
	#[serde(rename = "delayDays")]
	pub delay_days: Option<i32>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "dismissButtonLabel")]
	pub dismiss_button_label: Option<String>,
	#[serde(rename = "displayPosition")]
	pub display_position: Option<PromptDisplayPosition>,
	#[serde(rename = "displayType")]
	pub display_type: PromptDisplayType,
	#[serde(rename = "elementRelativePosition")]
	pub element_relative_position: Option<PromptElementRelativePosition>,
	#[serde(rename = "endDate")]
	pub end_date: Option<String>,
	#[serde(rename = "header")]
	pub header: Option<String>,
	#[serde(rename = "icon")]
	pub icon: Option<String>,
	#[serde(rename = "image")]
	pub image: Option<String>,
	#[serde(rename = "imageAltText")]
	pub image_alt_text: Option<String>,
	#[serde(rename = "imageLink")]
	pub image_link: Option<String>,
	#[serde(rename = "imageLocation")]
	pub image_location: Option<PromptImageLocation>,
	#[serde(rename = "indexWithIsPublished")]
	pub index_with_is_published: Option<String>,
	#[serde(rename = "indexWithoutIsPublished")]
	pub index_without_is_published: Option<String>,
	#[serde(rename = "isPublished")]
	pub is_published: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "publishedByUser")]
	pub published_by_user: Option<String>,
	#[serde(rename = "publishedDate")]
	pub published_date: Option<String>,
	#[serde(rename = "referenceElementContext")]
	pub reference_element_context: Option<String>,
	#[serde(rename = "shouldDisplayActionButton")]
	pub should_display_action_button: Option<bool>,
	#[serde(rename = "shouldIgnoreGlobalDelay")]
	pub should_ignore_global_delay: Option<bool>,
	#[serde(rename = "startDate")]
	pub start_date: Option<String>,
	#[serde(rename = "stepNumber")]
	pub step_number: Option<i32>,
	#[serde(rename = "targetAppDeveloperName")]
	pub target_app_developer_name: Option<String>,
	#[serde(rename = "targetAppNamespacePrefix")]
	pub target_app_namespace_prefix: Option<String>,
	#[serde(rename = "targetPageKey1")]
	pub target_page_key_1: Option<String>,
	#[serde(rename = "targetPageKey2")]
	pub target_page_key_2: Option<String>,
	#[serde(rename = "targetPageKey3")]
	pub target_page_key_3: Option<String>,
	#[serde(rename = "targetPageKey4")]
	pub target_page_key_4: Option<String>,
	#[serde(rename = "targetPageType")]
	pub target_page_type: Option<String>,
	#[serde(rename = "targetRecordType")]
	pub target_record_type: Option<String>,
	#[serde(rename = "themeColor")]
	pub theme_color: Option<PromptThemeColor>,
	#[serde(rename = "themeSaturation")]
	pub theme_saturation: Option<PromptThemeSaturation>,
	#[serde(rename = "timesToDisplay")]
	pub times_to_display: Option<i32>,
	#[serde(rename = "title")]
	pub title: String,
	#[serde(rename = "uiFormulaRule")]
	pub ui_formula_rule: Option<UiFormulaRule>,
	#[serde(rename = "userAccess")]
	pub user_access: Option<PromptUserAccess>,
	#[serde(rename = "userProfileAccess")]
	pub user_profile_access: Option<PromptUserProfileAccess>,
	#[serde(rename = "versionNumber")]
	pub version_number: i32,
	#[serde(rename = "videoLink")]
	pub video_link: Option<String>,
}
