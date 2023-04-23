use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Territory  {
	#[serde(rename = "accountAccessLevel")]
	pub account_access_level: Option<String>,
	#[serde(rename = "parentTerritory")]
	pub parent_territory: Option<String>,
}
