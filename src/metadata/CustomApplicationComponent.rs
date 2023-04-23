use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomApplicationComponent  {
	#[serde(rename = "buttonIconUrl")]
	pub button_icon_url: Option<String>,
	#[serde(rename = "buttonStyle")]
	pub button_style: Option<String>,
	#[serde(rename = "buttonText")]
	pub button_text: Option<String>,
	#[serde(rename = "buttonWidth")]
	pub button_width: Option<i32>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "isHeightFixed")]
	pub is_height_fixed: bool,
	#[serde(rename = "isHidden")]
	pub is_hidden: bool,
	#[serde(rename = "isWidthFixed")]
	pub is_width_fixed: bool,
	#[serde(rename = "visualforcePage")]
	pub visualforce_page: String,
	#[serde(rename = "width")]
	pub width: Option<i32>,
}
