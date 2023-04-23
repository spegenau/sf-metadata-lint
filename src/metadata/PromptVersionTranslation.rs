use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PromptVersionTranslation  {
	#[serde(rename = "actionButtonLabel")]
	pub action_button_label: Option<String>,
	#[serde(rename = "actionButtonLink")]
	pub action_button_link: Option<String>,
	#[serde(rename = "body")]
	pub body: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "dismissButtonLabel")]
	pub dismiss_button_label: Option<String>,
	#[serde(rename = "header")]
	pub header: Option<String>,
	#[serde(rename = "imageAltText")]
	pub image_alt_text: Option<String>,
	#[serde(rename = "imageLink")]
	pub image_link: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "stepNumber")]
	pub step_number: Option<i32>,
	#[serde(rename = "title")]
	pub title: Option<String>,
	#[serde(rename = "videoLink")]
	pub video_link: Option<String>,
}
