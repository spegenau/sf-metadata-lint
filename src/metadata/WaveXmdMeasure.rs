use crate::metadata::WaveXmdFormattingProperty::WaveXmdFormattingProperty;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdMeasure  {
	#[serde(rename = "conditionalFormatting")]
	pub conditional_formatting: Option<Vec<WaveXmdFormattingProperty>>,
	#[serde(rename = "currencies")]
	pub currencies: Option<Vec<Box<WaveXmdMeasure>>>,
	#[serde(rename = "currencyCode")]
	pub currency_code: Option<String>,
	#[serde(rename = "dateFormat")]
	pub date_format: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "formatCustomFormat")]
	pub format_custom_format: Option<String>,
	#[serde(rename = "formatDecimalDigits")]
	pub format_decimal_digits: Option<i32>,
	#[serde(rename = "formatDecimalSeparator")]
	pub format_decimal_separator: Option<String>,
	#[serde(rename = "formatIsNegativeParens")]
	pub format_is_negative_parens: Option<bool>,
	#[serde(rename = "formatPrefix")]
	pub format_prefix: Option<String>,
	#[serde(rename = "formatSuffix")]
	pub format_suffix: Option<String>,
	#[serde(rename = "formatThousandsSeparator")]
	pub format_thousands_separator: Option<String>,
	#[serde(rename = "formatUnit")]
	pub format_unit: Option<String>,
	#[serde(rename = "formatUnitMultiplier")]
	pub format_unit_multiplier: Option<f32>,
	#[serde(rename = "fullyQualifiedName")]
	pub fully_qualified_name: Option<String>,
	#[serde(rename = "isDerived")]
	pub is_derived: bool,
	#[serde(rename = "isMultiCurrency")]
	pub is_multi_currency: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "origin")]
	pub origin: Option<String>,
	#[serde(rename = "showDetailsDefaultFieldIndex")]
	pub show_details_default_field_index: Option<i32>,
	#[serde(rename = "showInExplorer")]
	pub show_in_explorer: Option<bool>,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
