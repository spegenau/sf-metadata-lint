use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SurveySettings  {
	#[serde(rename = "enableIndustriesCxmEnabled")]
	pub enable_industries_cxm_enabled: Option<bool>,
	#[serde(rename = "enableSurvey")]
	pub enable_survey: Option<bool>,
	#[serde(rename = "enableSurveyOwnerCanManageResponse")]
	pub enable_survey_owner_can_manage_response: Option<bool>,
}
