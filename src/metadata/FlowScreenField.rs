use crate::metadata::FlowDataType::FlowDataType;
use crate::metadata::FlowDataTypeMapping::FlowDataTypeMapping;
use crate::metadata::FlowElementReferenceOrValue::FlowElementReferenceOrValue;
use crate::metadata::FlowInputValidationRule::FlowInputValidationRule;
use crate::metadata::FlowRegionContainerType::FlowRegionContainerType;
use crate::metadata::FlowScreenFieldInputParameter::FlowScreenFieldInputParameter;
use crate::metadata::FlowScreenFieldInputsRevisited::FlowScreenFieldInputsRevisited;
use crate::metadata::FlowScreenFieldOutputParameter::FlowScreenFieldOutputParameter;
use crate::metadata::FlowScreenFieldType::FlowScreenFieldType;
use crate::metadata::FlowVisibilityRule::FlowVisibilityRule;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowScreenField  {
	#[serde(rename = "choiceReferences")]
	pub choice_references: Option<Vec<String>>,
	#[serde(rename = "dataType")]
	pub data_type: Option<FlowDataType>,
	#[serde(rename = "dataTypeMappings")]
	pub data_type_mappings: Option<Vec<FlowDataTypeMapping>>,
	#[serde(rename = "defaultSelectedChoiceReference")]
	pub default_selected_choice_reference: Option<String>,
	#[serde(rename = "defaultValue")]
	pub default_value: Option<FlowElementReferenceOrValue>,
	#[serde(rename = "extensionName")]
	pub extension_name: Option<String>,
	#[serde(rename = "fieldText")]
	pub field_text: Option<String>,
	#[serde(rename = "fieldType")]
	pub field_type: FlowScreenFieldType,
	#[serde(rename = "fields")]
	pub fields: Option<Vec<Box<FlowScreenField>>>,
	#[serde(rename = "helpText")]
	pub help_text: Option<String>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowScreenFieldInputParameter>>,
	#[serde(rename = "inputsOnNextNavToAssocScrn")]
	pub inputs_on_next_nav_to_assoc_scrn: Option<FlowScreenFieldInputsRevisited>,
	#[serde(rename = "isRequired")]
	pub is_required: Option<bool>,
	#[serde(rename = "isVisible")]
	pub is_visible: Option<bool>,
	#[serde(rename = "objectFieldReference")]
	pub object_field_reference: Option<String>,
	#[serde(rename = "outputParameters")]
	pub output_parameters: Option<Vec<FlowScreenFieldOutputParameter>>,
	#[serde(rename = "regionContainerType")]
	pub region_container_type: Option<FlowRegionContainerType>,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
	#[serde(rename = "storeOutputAutomatically")]
	pub store_output_automatically: Option<bool>,
	#[serde(rename = "validationRule")]
	pub validation_rule: Option<FlowInputValidationRule>,
	#[serde(rename = "visibilityRule")]
	pub visibility_rule: Option<FlowVisibilityRule>,
}
