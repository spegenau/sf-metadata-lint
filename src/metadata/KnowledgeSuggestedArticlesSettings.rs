use crate::metadata::KnowledgeCaseFieldsSettings::KnowledgeCaseFieldsSettings;
use crate::metadata::KnowledgeWorkOrderFieldsSettings::KnowledgeWorkOrderFieldsSettings;
use crate::metadata::KnowledgeWorkOrderLineItemFieldsSettings::KnowledgeWorkOrderLineItemFieldsSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct KnowledgeSuggestedArticlesSettings  {
	#[serde(rename = "caseFields")]
	pub case_fields: Option<KnowledgeCaseFieldsSettings>,
	#[serde(rename = "useSuggestedArticlesForCase")]
	pub use_suggested_articles_for_case: Option<bool>,
	#[serde(rename = "workOrderFields")]
	pub work_order_fields: Option<KnowledgeWorkOrderFieldsSettings>,
	#[serde(rename = "workOrderLineItemFields")]
	pub work_order_line_item_fields: Option<KnowledgeWorkOrderLineItemFieldsSettings>,
}
