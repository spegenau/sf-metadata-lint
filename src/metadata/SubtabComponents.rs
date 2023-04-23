use crate::metadata::Container::Container;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SubtabComponents  {
	#[serde(rename = "containers")]
	pub containers: Option<Vec<Container>>,
}
