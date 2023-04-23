use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdDate  {
	#[serde(rename = "alias")]
	pub alias: String,
	#[serde(rename = "compact")]
	pub compact: Option<bool>,
	#[serde(rename = "dateFieldDay")]
	pub date_field_day: Option<String>,
	#[serde(rename = "dateFieldEpochDay")]
	pub date_field_epoch_day: Option<String>,
	#[serde(rename = "dateFieldEpochSecond")]
	pub date_field_epoch_second: Option<String>,
	#[serde(rename = "dateFieldFiscalMonth")]
	pub date_field_fiscal_month: Option<String>,
	#[serde(rename = "dateFieldFiscalQuarter")]
	pub date_field_fiscal_quarter: Option<String>,
	#[serde(rename = "dateFieldFiscalWeek")]
	pub date_field_fiscal_week: Option<String>,
	#[serde(rename = "dateFieldFiscalYear")]
	pub date_field_fiscal_year: Option<String>,
	#[serde(rename = "dateFieldFullYear")]
	pub date_field_full_year: Option<String>,
	#[serde(rename = "dateFieldHour")]
	pub date_field_hour: Option<String>,
	#[serde(rename = "dateFieldMinute")]
	pub date_field_minute: Option<String>,
	#[serde(rename = "dateFieldMonth")]
	pub date_field_month: Option<String>,
	#[serde(rename = "dateFieldQuarter")]
	pub date_field_quarter: Option<String>,
	#[serde(rename = "dateFieldSecond")]
	pub date_field_second: Option<String>,
	#[serde(rename = "dateFieldWeek")]
	pub date_field_week: Option<String>,
	#[serde(rename = "dateFieldYear")]
	pub date_field_year: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "firstDayOfWeek")]
	pub first_day_of_week: i32,
	#[serde(rename = "fiscalMonthOffset")]
	pub fiscal_month_offset: i32,
	#[serde(rename = "isYearEndFiscalYear")]
	pub is_year_end_fiscal_year: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "showInExplorer")]
	pub show_in_explorer: Option<bool>,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
	#[serde(rename = "type")]
	pub _type: String,
}
