use crate::metadata::FiscalYearSettings::FiscalYearSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CompanySettings  {
	#[serde(rename = "enableCustomFiscalYear")]
	pub enable_custom_fiscal_year: bool,
	#[serde(rename = "fiscalYear")]
	pub fiscal_year: Option<FiscalYearSettings>,
}
