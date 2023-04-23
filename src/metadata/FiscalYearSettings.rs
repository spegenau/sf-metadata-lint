use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FiscalYearSettings  {
	#[serde(rename = "fiscalYearNameBasedOn")]
	pub fiscal_year_name_based_on: Option<String>,
	#[serde(rename = "startMonth")]
	pub start_month: Option<String>,
}
