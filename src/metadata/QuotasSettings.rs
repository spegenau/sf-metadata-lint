use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct QuotasSettings  {
	#[serde(rename = "showQuotas")]
	pub show_quotas: bool,
}
