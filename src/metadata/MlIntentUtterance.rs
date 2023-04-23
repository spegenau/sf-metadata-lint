use crate::metadata::Language::Language;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct MlIntentUtterance  {
	#[serde(rename = "language")]
	pub language: Option<Language>,
	#[serde(rename = "utterance")]
	pub utterance: String,
}
