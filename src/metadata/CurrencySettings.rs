use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CurrencySettings  {
	#[serde(rename = "enableCurrencyEffectiveDates")]
	pub enable_currency_effective_dates: Option<bool>,
	#[serde(rename = "enableCurrencySymbolWithMultiCurrency")]
	pub enable_currency_symbol_with_multi_currency: Option<bool>,
	#[serde(rename = "enableMultiCurrency")]
	pub enable_multi_currency: Option<bool>,
	#[serde(rename = "isMultiCurrencyActivationAllowed")]
	pub is_multi_currency_activation_allowed: Option<bool>,
	#[serde(rename = "isParenCurrencyConvDisabled")]
	pub is_paren_currency_conv_disabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
