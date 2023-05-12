use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuoteSettings  {
	#[serde(rename = "enableQuote")]
	pub enable_quote: bool,
	#[serde(rename = "enableQuotesWithoutOppEnabled")]
	pub enable_quotes_without_opp_enabled: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
