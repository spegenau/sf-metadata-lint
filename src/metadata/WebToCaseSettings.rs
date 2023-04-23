use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WebToCaseSettings  {
	#[serde(rename = "caseOrigin")]
	pub case_origin: Option<String>,
	#[serde(rename = "defaultResponseTemplate")]
	pub default_response_template: Option<String>,
	#[serde(rename = "enableWebToCase")]
	pub enable_web_to_case: Option<bool>,
}
