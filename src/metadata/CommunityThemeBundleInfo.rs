use crate::metadata::CommunityTemplateBundleInfoType::CommunityTemplateBundleInfoType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CommunityThemeBundleInfo  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "image")]
	pub image: Option<String>,
	#[serde(rename = "order")]
	pub order: i32,
	#[serde(rename = "title")]
	pub title: String,
	#[serde(rename = "type")]
	pub _type: CommunityTemplateBundleInfoType,
}
