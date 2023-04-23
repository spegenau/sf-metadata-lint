use crate::metadata::LogCategory::LogCategory;
use crate::metadata::LogCategoryLevel::LogCategoryLevel;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LogInfo  {
	#[serde(rename = "category")]
	pub category: LogCategory,
	#[serde(rename = "level")]
	pub level: LogCategoryLevel,
}
