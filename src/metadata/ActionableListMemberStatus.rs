use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ActionableListMemberStatus  {
	#[serde(rename = "iconName")]
	pub icon_name: Option<String>,
	#[serde(rename = "status")]
	pub status: Option<String>,
}
