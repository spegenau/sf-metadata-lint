use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct IncludeEstTaxInQuoteSettings  {
	#[serde(rename = "enableQuoteEstimatedTax")]
	pub enable_quote_estimated_tax: Option<bool>,
}
