use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdFormattingBin  {
	#[serde(rename = "bin")]
	pub bin: String,
	#[serde(rename = "formatValue")]
	pub format_value: String,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
