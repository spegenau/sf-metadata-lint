use crate::metadata::CustomFieldTranslation::CustomFieldTranslation;
use crate::metadata::FieldSetTranslation::FieldSetTranslation;
use crate::metadata::Gender::Gender;
use crate::metadata::LayoutTranslation::LayoutTranslation;
use crate::metadata::ObjectNameCaseValue::ObjectNameCaseValue;
use crate::metadata::QuickActionTranslation::QuickActionTranslation;
use crate::metadata::RecordTypeTranslation::RecordTypeTranslation;
use crate::metadata::SharingReasonTranslation::SharingReasonTranslation;
use crate::metadata::StandardFieldTranslation::StandardFieldTranslation;
use crate::metadata::StartsWith::StartsWith;
use crate::metadata::ValidationRuleTranslation::ValidationRuleTranslation;
use crate::metadata::WebLinkTranslation::WebLinkTranslation;
use crate::metadata::WorkflowTaskTranslation::WorkflowTaskTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomObjectTranslation  {
	#[serde(rename = "caseValues")]
	pub case_values: Option<Vec<ObjectNameCaseValue>>,
	#[serde(rename = "fieldSets")]
	pub field_sets: Option<Vec<FieldSetTranslation>>,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<CustomFieldTranslation>>,
	#[serde(rename = "gender")]
	pub gender: Option<Gender>,
	#[serde(rename = "layouts")]
	pub layouts: Option<Vec<LayoutTranslation>>,
	#[serde(rename = "nameFieldLabel")]
	pub name_field_label: Option<String>,
	#[serde(rename = "quickActions")]
	pub quick_actions: Option<Vec<QuickActionTranslation>>,
	#[serde(rename = "recordTypes")]
	pub record_types: Option<Vec<RecordTypeTranslation>>,
	#[serde(rename = "sharingReasons")]
	pub sharing_reasons: Option<Vec<SharingReasonTranslation>>,
	#[serde(rename = "standardFields")]
	pub standard_fields: Option<Vec<StandardFieldTranslation>>,
	#[serde(rename = "startsWith")]
	pub starts_with: Option<StartsWith>,
	#[serde(rename = "validationRules")]
	pub validation_rules: Option<Vec<ValidationRuleTranslation>>,
	#[serde(rename = "webLinks")]
	pub web_links: Option<Vec<WebLinkTranslation>>,
	#[serde(rename = "workflowTasks")]
	pub workflow_tasks: Option<Vec<WorkflowTaskTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
