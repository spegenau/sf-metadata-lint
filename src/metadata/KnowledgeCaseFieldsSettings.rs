use crate::metadata::KnowledgeCaseField::KnowledgeCaseField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeCaseFieldsSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<KnowledgeCaseField>>,
}
