use crate::metadata::DeleteConstraint::DeleteConstraint;
use crate::metadata::EncryptedFieldMaskChar::EncryptedFieldMaskChar;
use crate::metadata::EncryptedFieldMaskType::EncryptedFieldMaskType;
use crate::metadata::EncryptionScheme::EncryptionScheme;
use crate::metadata::FieldManageability::FieldManageability;
use crate::metadata::FieldType::FieldType;
use crate::metadata::FilterItem::FilterItem;
use crate::metadata::LookupFilter::LookupFilter;
use crate::metadata::MktDataLakeFieldAttributes::MktDataLakeFieldAttributes;
use crate::metadata::MktDataModelFieldAttributes::MktDataModelFieldAttributes;
use crate::metadata::SummaryOperations::SummaryOperations;
use crate::metadata::TreatBlanksAs::TreatBlanksAs;
use crate::metadata::ValueSet::ValueSet;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CustomField  {
	#[serde(rename = "businessOwnerGroup")]
	pub business_owner_group: Option<String>,
	#[serde(rename = "businessOwnerUser")]
	pub business_owner_user: Option<String>,
	#[serde(rename = "businessStatus")]
	pub business_status: Option<String>,
	#[serde(rename = "caseSensitive")]
	pub case_sensitive: Option<bool>,
	#[serde(rename = "complianceGroup")]
	pub compliance_group: Option<String>,
	#[serde(rename = "customDataType")]
	pub custom_data_type: Option<String>,
	#[serde(rename = "defaultValue")]
	pub default_value: Option<String>,
	#[serde(rename = "deleteConstraint")]
	pub delete_constraint: Option<DeleteConstraint>,
	#[serde(rename = "deprecated")]
	pub deprecated: Option<bool>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "displayFormat")]
	pub display_format: Option<String>,
	#[serde(rename = "encryptionScheme")]
	pub encryption_scheme: Option<EncryptionScheme>,
	#[serde(rename = "escapeMarkup")]
	pub escape_markup: Option<bool>,
	#[serde(rename = "externalDeveloperName")]
	pub external_developer_name: Option<String>,
	#[serde(rename = "externalId")]
	pub external_id: Option<bool>,
	#[serde(rename = "fieldManageability")]
	pub field_manageability: Option<FieldManageability>,
	#[serde(rename = "formula")]
	pub formula: Option<String>,
	#[serde(rename = "formulaTreatBlanksAs")]
	pub formula_treat_blanks_as: Option<TreatBlanksAs>,
	#[serde(rename = "inlineHelpText")]
	pub inline_help_text: Option<String>,
	#[serde(rename = "isAIPredictionField")]
	pub is_ai_prediction_field: Option<bool>,
	#[serde(rename = "isConvertLeadDisabled")]
	pub is_convert_lead_disabled: Option<bool>,
	#[serde(rename = "isFilteringDisabled")]
	pub is_filtering_disabled: Option<bool>,
	#[serde(rename = "isNameField")]
	pub is_name_field: Option<bool>,
	#[serde(rename = "isSortingDisabled")]
	pub is_sorting_disabled: Option<bool>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "length")]
	pub length: Option<i32>,
	#[serde(rename = "lookupFilter")]
	pub lookup_filter: Option<LookupFilter>,
	#[serde(rename = "maskChar")]
	pub mask_char: Option<EncryptedFieldMaskChar>,
	#[serde(rename = "maskType")]
	pub mask_type: Option<EncryptedFieldMaskType>,
	#[serde(rename = "metadataRelationshipControllingField")]
	pub metadata_relationship_controlling_field: Option<String>,
	#[serde(rename = "mktDataLakeFieldAttributes")]
	pub mkt_data_lake_field_attributes: Option<MktDataLakeFieldAttributes>,
	#[serde(rename = "mktDataModelFieldAttributes")]
	pub mkt_data_model_field_attributes: Option<MktDataModelFieldAttributes>,
	#[serde(rename = "populateExistingRows")]
	pub populate_existing_rows: Option<bool>,
	#[serde(rename = "precision")]
	pub precision: Option<i32>,
	#[serde(rename = "referenceTargetField")]
	pub reference_target_field: Option<String>,
	#[serde(rename = "referenceTo")]
	pub reference_to: Option<String>,
	#[serde(rename = "relationshipLabel")]
	pub relationship_label: Option<String>,
	#[serde(rename = "relationshipName")]
	pub relationship_name: Option<String>,
	#[serde(rename = "relationshipOrder")]
	pub relationship_order: Option<i32>,
	#[serde(rename = "reparentableMasterDetail")]
	pub reparentable_master_detail: Option<bool>,
	#[serde(rename = "required")]
	pub required: Option<bool>,
	#[serde(rename = "restrictedAdminField")]
	pub restricted_admin_field: Option<bool>,
	#[serde(rename = "scale")]
	pub scale: Option<i32>,
	#[serde(rename = "securityClassification")]
	pub security_classification: Option<String>,
	#[serde(rename = "startingNumber")]
	pub starting_number: Option<i32>,
	#[serde(rename = "stripMarkup")]
	pub strip_markup: Option<bool>,
	#[serde(rename = "summarizedField")]
	pub summarized_field: Option<String>,
	#[serde(rename = "summaryFilterItems")]
	pub summary_filter_items: Option<Vec<FilterItem>>,
	#[serde(rename = "summaryForeignKey")]
	pub summary_foreign_key: Option<String>,
	#[serde(rename = "summaryOperation")]
	pub summary_operation: Option<SummaryOperations>,
	#[serde(rename = "trackFeedHistory")]
	pub track_feed_history: Option<bool>,
	#[serde(rename = "trackHistory")]
	pub track_history: Option<bool>,
	#[serde(rename = "trackTrending")]
	pub track_trending: Option<bool>,
	#[serde(rename = "translateData")]
	pub translate_data: Option<bool>,
	#[serde(rename = "type")]
	pub _type: Option<FieldType>,
	#[serde(rename = "unique")]
	pub unique: Option<bool>,
	#[serde(rename = "valueSet")]
	pub value_set: Option<ValueSet>,
	#[serde(rename = "visibleLines")]
	pub visible_lines: Option<i32>,
	#[serde(rename = "writeRequiresMasterRead")]
	pub write_requires_master_read: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
