use crate::metadata::ValueTranslation::ValueTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct GlobalValueSetTranslation  {
	#[serde(rename = "valueTranslation")]
	pub value_translation: Option<Vec<ValueTranslation>>,
}