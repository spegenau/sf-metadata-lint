use crate::metadata::KnowledgeWorkOrderLineItemField::KnowledgeWorkOrderLineItemField;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeWorkOrderLineItemFieldsSettings  {
	#[serde(rename = "field")]
	pub field: Option<Vec<KnowledgeWorkOrderLineItemField>>,
}
