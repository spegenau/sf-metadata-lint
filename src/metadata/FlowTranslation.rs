use crate::metadata::FlowChoiceTranslation::FlowChoiceTranslation;
use crate::metadata::FlowScreenTranslation::FlowScreenTranslation;
use crate::metadata::FlowStageTranslation::FlowStageTranslation;
use crate::metadata::FlowTextTemplateTranslation::FlowTextTemplateTranslation;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowTranslation  {
	#[serde(rename = "choices")]
	pub choices: Option<Vec<FlowChoiceTranslation>>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
	#[serde(rename = "label")]
	pub label: Option<String>,
	#[serde(rename = "screens")]
	pub screens: Option<Vec<FlowScreenTranslation>>,
	#[serde(rename = "stages")]
	pub stages: Option<Vec<FlowStageTranslation>>,
	#[serde(rename = "textTemplates")]
	pub text_templates: Option<Vec<FlowTextTemplateTranslation>>,
}
