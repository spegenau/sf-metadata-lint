use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Keyword  {
	#[serde(rename = "keyword")]
	pub keyword: String,
}
