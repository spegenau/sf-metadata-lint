use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdRecordDisplayLookup  {
	#[serde(rename = "recordDisplayField")]
	pub record_display_field: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
