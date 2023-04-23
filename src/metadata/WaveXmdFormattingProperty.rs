use crate::metadata::WaveXmdFormattingBin::WaveXmdFormattingBin;
use crate::metadata::WaveXmdFormattingPredicate::WaveXmdFormattingPredicate;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdFormattingProperty  {
	#[serde(rename = "formattingBins")]
	pub formatting_bins: Option<Vec<WaveXmdFormattingBin>>,
	#[serde(rename = "formattingPredicates")]
	pub formatting_predicates: Option<Vec<WaveXmdFormattingPredicate>>,
	#[serde(rename = "property")]
	pub property: String,
	#[serde(rename = "referenceField")]
	pub reference_field: String,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
	#[serde(rename = "type")]
	pub _type: String,
}
