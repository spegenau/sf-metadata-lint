use crate::metadata::Metadata::Metadata;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ReadResult  {
	#[serde(rename = "records")]
	pub records: Option<Vec<Metadata>>,
}
