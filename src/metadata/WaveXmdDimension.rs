use crate::metadata::WaveXmdDimensionCustomAction::WaveXmdDimensionCustomAction;
use crate::metadata::WaveXmdDimensionMember::WaveXmdDimensionMember;
use crate::metadata::WaveXmdDimensionSalesforceAction::WaveXmdDimensionSalesforceAction;
use crate::metadata::WaveXmdFormattingProperty::WaveXmdFormattingProperty;
use crate::metadata::WaveXmdRecordDisplayLookup::WaveXmdRecordDisplayLookup;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct WaveXmdDimension  {
	#[serde(rename = "conditionalFormatting")]
	pub conditional_formatting: Option<Vec<WaveXmdFormattingProperty>>,
	#[serde(rename = "customActions")]
	pub custom_actions: Option<Vec<WaveXmdDimensionCustomAction>>,
	#[serde(rename = "customActionsEnabled")]
	pub custom_actions_enabled: Option<bool>,
	#[serde(rename = "dateFormat")]
	pub date_format: Option<String>,
	#[serde(rename = "defaultAction")]
	pub default_action: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "field")]
	pub field: String,
	#[serde(rename = "fullyQualifiedName")]
	pub fully_qualified_name: Option<String>,
	#[serde(rename = "imageTemplate")]
	pub image_template: Option<String>,
	#[serde(rename = "isDerived")]
	pub is_derived: bool,
	#[serde(rename = "isMultiValue")]
	pub is_multi_value: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "linkTemplate")]
	pub link_template: Option<String>,
	#[serde(rename = "linkTemplateEnabled")]
	pub link_template_enabled: Option<bool>,
	#[serde(rename = "linkTooltip")]
	pub link_tooltip: Option<String>,
	#[serde(rename = "members")]
	pub members: Option<Vec<WaveXmdDimensionMember>>,
	#[serde(rename = "origin")]
	pub origin: Option<String>,
	#[serde(rename = "recordDisplayFields")]
	pub record_display_fields: Option<Vec<WaveXmdRecordDisplayLookup>>,
	#[serde(rename = "recordIdField")]
	pub record_id_field: Option<String>,
	#[serde(rename = "recordOrganizationIdField")]
	pub record_organization_id_field: Option<String>,
	#[serde(rename = "salesforceActions")]
	pub salesforce_actions: Option<Vec<WaveXmdDimensionSalesforceAction>>,
	#[serde(rename = "salesforceActionsEnabled")]
	pub salesforce_actions_enabled: Option<bool>,
	#[serde(rename = "showDetailsDefaultFieldIndex")]
	pub show_details_default_field_index: Option<i32>,
	#[serde(rename = "showInExplorer")]
	pub show_in_explorer: Option<bool>,
	#[serde(rename = "sortIndex")]
	pub sort_index: i32,
}
