use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct HomePageLayout  {
	#[serde(rename = "narrowComponents")]
	pub narrow_components: Option<Vec<String>>,
	#[serde(rename = "wideComponents")]
	pub wide_components: Option<Vec<String>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
