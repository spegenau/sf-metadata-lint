use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InvLatePymntRiskCalcSettings  {
	#[serde(rename = "enableInvLatePymntRiskCalc")]
	pub enable_inv_late_pymnt_risk_calc: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
