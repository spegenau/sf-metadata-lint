use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PaymentsIngestEnabledSettings  {
	#[serde(rename = "paymentsIngestEnabled")]
	pub payments_ingest_enabled: Option<bool>,
}
