use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InterestTaggingSettings  {
	#[serde(rename = "enableInterestTagging")]
	pub enable_interest_tagging: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
