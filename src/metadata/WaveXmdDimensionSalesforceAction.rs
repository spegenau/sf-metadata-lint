use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdDimensionSalesforceAction  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "salesforceActionName")]
	pub salesforce_action_name: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
