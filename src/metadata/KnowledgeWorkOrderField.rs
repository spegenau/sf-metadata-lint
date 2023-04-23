use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeWorkOrderField  {
	#[serde(rename = "name")]
	pub name: Option<String>,
}
