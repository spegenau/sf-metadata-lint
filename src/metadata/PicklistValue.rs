use crate::metadata::ForecastCategories::ForecastCategories;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PicklistValue  {
	#[serde(rename = "color")]
	pub color: Option<String>,
	#[serde(rename = "default")]
	pub default: bool,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "allowEmail")]
	pub allow_email: Option<bool>,
	#[serde(rename = "closed")]
	pub closed: Option<bool>,
	#[serde(rename = "controllingFieldValues")]
	pub controlling_field_values: Option<Vec<String>>,
	#[serde(rename = "converted")]
	pub converted: Option<bool>,
	#[serde(rename = "cssExposed")]
	pub css_exposed: Option<bool>,
	#[serde(rename = "forecastCategory")]
	pub forecast_category: Option<ForecastCategories>,
	#[serde(rename = "highPriority")]
	pub high_priority: Option<bool>,
	#[serde(rename = "probability")]
	pub probability: Option<i32>,
	#[serde(rename = "reverseRole")]
	pub reverse_role: Option<String>,
	#[serde(rename = "reviewed")]
	pub reviewed: Option<bool>,
	#[serde(rename = "won")]
	pub won: Option<bool>,
}
