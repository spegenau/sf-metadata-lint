use crate::metadata::KnowledgeWorkOrderField::KnowledgeWorkOrderField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeWorkOrderFieldsSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<KnowledgeWorkOrderField>>,
}
