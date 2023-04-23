use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OmniProcessElement  {
	#[serde(rename = "childElements")]
	pub child_elements: Option<Vec<Box<OmniProcessElement>>>,
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "designerCustomizationType")]
	pub designer_customization_type: Option<String>,
	#[serde(rename = "embeddedOmniScriptKey")]
	pub embedded_omni_script_key: Option<String>,
	#[serde(rename = "isActive")]
	pub is_active: Option<bool>,
	#[serde(rename = "isOmniScriptEmbeddable")]
	pub is_omni_script_embeddable: Option<bool>,
	#[serde(rename = "level")]
	pub level: Option<f32>,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "omniProcessVersionNumber")]
	pub omni_process_version_number: Option<f32>,
	#[serde(rename = "parentElementName")]
	pub parent_element_name: Option<String>,
	#[serde(rename = "parentElementType")]
	pub parent_element_type: Option<String>,
	#[serde(rename = "propertySetConfig")]
	pub property_set_config: Option<String>,
	#[serde(rename = "questionDevName")]
	pub question_dev_name: Option<String>,
	#[serde(rename = "sequenceNumber")]
	pub sequence_number: Option<f32>,
	#[serde(rename = "type")]
	pub _type: Option<String>,
}
