use crate::metadata::LetterheadHorizontalAlignment::LetterheadHorizontalAlignment;
use crate::metadata::LetterheadVerticalAlignment::LetterheadVerticalAlignment;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LetterheadHeaderFooter  {
	#[serde(rename = "backgroundColor")]
	pub background_color: String,
	#[serde(rename = "height")]
	pub height: i32,
	#[serde(rename = "horizontalAlignment")]
	pub horizontal_alignment: Option<LetterheadHorizontalAlignment>,
	#[serde(rename = "logo")]
	pub logo: Option<String>,
	#[serde(rename = "verticalAlignment")]
	pub vertical_alignment: Option<LetterheadVerticalAlignment>,
}
