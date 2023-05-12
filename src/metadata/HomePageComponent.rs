use crate::metadata::PageComponentType::PageComponentType;
use crate::metadata::PageComponentWidth::PageComponentWidth;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct HomePageComponent  {
	#[serde(rename = "body")]
	pub body: Option<String>,
	#[serde(rename = "height")]
	pub height: Option<i32>,
	#[serde(rename = "links")]
	pub links: Option<Vec<String>>,
	#[serde(rename = "page")]
	pub page: Option<String>,
	#[serde(rename = "pageComponentType")]
	pub page_component_type: PageComponentType,
	#[serde(rename = "showLabel")]
	pub show_label: Option<bool>,
	#[serde(rename = "showScrollbars")]
	pub show_scrollbars: Option<bool>,
	#[serde(rename = "width")]
	pub width: Option<PageComponentWidth>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
