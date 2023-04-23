use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LetterheadLine  {
	#[serde(rename = "color")]
	pub color: String,
	#[serde(rename = "height")]
	pub height: i32,
}
