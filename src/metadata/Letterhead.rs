use crate::metadata::LetterheadHeaderFooter::LetterheadHeaderFooter;
use crate::metadata::LetterheadLine::LetterheadLine;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Letterhead  {
	#[serde(rename = "available")]
	pub available: bool,
	#[serde(rename = "backgroundColor")]
	pub background_color: String,
	#[serde(rename = "bodyColor")]
	pub body_color: String,
	#[serde(rename = "bottomLine")]
	pub bottom_line: LetterheadLine,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "footer")]
	pub footer: LetterheadHeaderFooter,
	#[serde(rename = "header")]
	pub header: LetterheadHeaderFooter,
	#[serde(rename = "middleLine")]
	pub middle_line: LetterheadLine,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "topLine")]
	pub top_line: LetterheadLine,
}
