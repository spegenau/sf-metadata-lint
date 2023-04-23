use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct TabLimitConfig  {
	#[serde(rename = "maxNumberOfPrimaryTabs")]
	pub max_number_of_primary_tabs: Option<String>,
	#[serde(rename = "maxNumberOfSubTabs")]
	pub max_number_of_sub_tabs: Option<String>,
}
