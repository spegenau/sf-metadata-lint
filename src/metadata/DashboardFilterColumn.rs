use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardFilterColumn  {
	#[serde(rename = "column")]
	pub column: String,
}
