use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReportHistoricalSelector  {
	#[serde(rename = "snapshot")]
	pub snapshot: Option<Vec<String>>,
}
