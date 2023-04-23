use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdFormattingPredicate  {
	#[serde(rename = "formatValue")]
	pub format_value: String,
	#[serde(rename = "operator")]
	pub operator: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
	#[serde(rename = "value")]
	pub value: String,
}
