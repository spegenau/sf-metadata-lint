use crate::metadata::ForecastCategories::ForecastCategories;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct StandardValue  {
	#[serde(rename = "allowEmail")]
	pub allow_email: Option<bool>,
	#[serde(rename = "closed")]
	pub closed: Option<bool>,
	#[serde(rename = "converted")]
	pub converted: Option<bool>,
	#[serde(rename = "cssExposed")]
	pub css_exposed: Option<bool>,
	#[serde(rename = "forecastCategory")]
	pub forecast_category: Option<ForecastCategories>,
	#[serde(rename = "groupingString")]
	pub grouping_string: Option<String>,
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
