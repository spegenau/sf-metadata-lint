use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IndustriesManufacturingSettings  {
	#[serde(rename = "enableIndManufacturing")]
	pub enable_ind_manufacturing: Option<bool>,
	#[serde(rename = "enableIndustriesMfgAccountForecast")]
	pub enable_industries_mfg_account_forecast: Option<bool>,
	#[serde(rename = "enableIndustriesMfgAdvForecast")]
	pub enable_industries_mfg_adv_forecast: Option<bool>,
	#[serde(rename = "enableIndustriesMfgIAS")]
	pub enable_industries_mfg_ias: Option<bool>,
	#[serde(rename = "enableIndustriesMfgProgram")]
	pub enable_industries_mfg_program: Option<bool>,
	#[serde(rename = "enableIndustriesMfgTargets")]
	pub enable_industries_mfg_targets: Option<bool>,
	#[serde(rename = "enablePartnerLeadManagement")]
	pub enable_partner_lead_management: Option<bool>,
	#[serde(rename = "enablePartnerPerformanceManagement")]
	pub enable_partner_performance_management: Option<bool>,
	#[serde(rename = "enablePartnerVisitManagement")]
	pub enable_partner_visit_management: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
