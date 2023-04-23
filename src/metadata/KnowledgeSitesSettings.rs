use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeSitesSettings  {
	#[serde(rename = "site")]
	pub site: Option<Vec<String>>,
}
