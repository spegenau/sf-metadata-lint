use crate::metadata::ODTItemFilterDataType::ODTItemFilterDataType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniDataTransformItem  {
	#[serde(rename = "defaultValue")]
	pub default_value: Option<String>,
	#[serde(rename = "disabled")]
	pub disabled: Option<bool>,
	#[serde(rename = "filterDataType")]
	pub filter_data_type: Option<ODTItemFilterDataType>,
	#[serde(rename = "filterGroup")]
	pub filter_group: Option<f32>,
	#[serde(rename = "filterOperator")]
	pub filter_operator: Option<String>,
	#[serde(rename = "filterValue")]
	pub filter_value: Option<String>,
	#[serde(rename = "formulaConverted")]
	pub formula_converted: Option<String>,
	#[serde(rename = "formulaExpression")]
	pub formula_expression: Option<String>,
	#[serde(rename = "formulaResultPath")]
	pub formula_result_path: Option<String>,
	#[serde(rename = "formulaSequence")]
	pub formula_sequence: Option<f32>,
	#[serde(rename = "globalKey")]
	pub global_key: Option<String>,
	#[serde(rename = "inputFieldName")]
	pub input_field_name: Option<String>,
	#[serde(rename = "inputObjectName")]
	pub input_object_name: Option<String>,
	#[serde(rename = "inputObjectQuerySequence")]
	pub input_object_query_sequence: Option<f32>,
	#[serde(rename = "linkedFieldName")]
	pub linked_field_name: Option<String>,
	#[serde(rename = "linkedObjectSequence")]
	pub linked_object_sequence: Option<f32>,
	#[serde(rename = "lookupByFieldName")]
	pub lookup_by_field_name: Option<String>,
	#[serde(rename = "lookupObjectName")]
	pub lookup_object_name: Option<String>,
	#[serde(rename = "lookupReturnedFieldName")]
	pub lookup_returned_field_name: Option<String>,
	#[serde(rename = "migrationAttribute")]
	pub migration_attribute: Option<String>,
	#[serde(rename = "migrationCategory")]
	pub migration_category: Option<String>,
	#[serde(rename = "migrationGroup")]
	pub migration_group: Option<String>,
	#[serde(rename = "migrationKey")]
	pub migration_key: Option<String>,
	#[serde(rename = "migrationPattern")]
	pub migration_pattern: Option<String>,
	#[serde(rename = "migrationProcess")]
	pub migration_process: Option<String>,
	#[serde(rename = "migrationType")]
	pub migration_type: Option<String>,
	#[serde(rename = "migrationValue")]
	pub migration_value: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "omniDataTransformation")]
	pub omni_data_transformation: Option<String>,
	#[serde(rename = "omniDataTransformationId")]
	pub omni_data_transformation_id: Option<String>,
	#[serde(rename = "outputCreationSequence")]
	pub output_creation_sequence: Option<f32>,
	#[serde(rename = "outputFieldFormat")]
	pub output_field_format: Option<String>,
	#[serde(rename = "outputFieldName")]
	pub output_field_name: Option<String>,
	#[serde(rename = "outputObjectName")]
	pub output_object_name: Option<String>,
	#[serde(rename = "requiredForUpsert")]
	pub required_for_upsert: Option<bool>,
	#[serde(rename = "transformValuesMappings")]
	pub transform_values_mappings: Option<String>,
	#[serde(rename = "upsertKey")]
	pub upsert_key: Option<bool>,
}