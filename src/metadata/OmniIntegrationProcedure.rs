use crate::metadata::OmniProcessElement::OmniProcessElement;
use crate::metadata::OmniProcessType::OmniProcessType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniIntegrationProcedure  {
	#[serde(rename = "customHtmlTemplates")]
	pub custom_html_templates: Option<String>,
	#[serde(rename = "customJavaScript")]
	pub custom_java_script: Option<String>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "designerCustomizationType")]
	pub designer_customization_type: Option<String>,
	#[serde(rename = "elementTypeComponentMapping")]
	pub element_type_component_mapping: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isIntegrationProcedure")]
	pub is_integration_procedure: Option<bool>,
	#[serde(rename = "isMetadataCacheDisabled")]
	pub is_metadata_cache_disabled: Option<bool>,
	#[serde(rename = "isOmniScriptEmbeddable")]
	pub is_omni_script_embeddable: Option<bool>,
	#[serde(rename = "isTestProcedure")]
	pub is_test_procedure: Option<bool>,
	#[serde(rename = "isWebCompEnabled")]
	pub is_web_comp_enabled: Option<bool>,
	#[serde(rename = "language")]
	pub language: String,
	#[serde(rename = "lastPreviewPage")]
	pub last_preview_page: Option<String>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "nameSpace")]
	pub name_space: Option<String>,
	#[serde(rename = "omniProcessElements")]
	pub omni_process_elements: Option<Vec<OmniProcessElement>>,
	#[serde(rename = "omniProcessKey")]
	pub omni_process_key: Option<String>,
	#[serde(rename = "omniProcessType")]
	pub omni_process_type: OmniProcessType,
	#[serde(rename = "overrideKey")]
	pub override_key: Option<String>,
	#[serde(rename = "propertySetConfig")]
	pub property_set_config: Option<String>,
	#[serde(rename = "requiredPermission")]
	pub required_permission: Option<String>,
	#[serde(rename = "responseCacheType")]
	pub response_cache_type: Option<String>,
	#[serde(rename = "subType")]
	pub sub_type: String,
	#[serde(rename = "type")]
	pub _type: String,
	#[serde(rename = "uniqueName")]
	pub unique_name: String,
	#[serde(rename = "versionNumber")]
	pub version_number: f32,
	#[serde(rename = "webComponentKey")]
	pub web_component_key: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
