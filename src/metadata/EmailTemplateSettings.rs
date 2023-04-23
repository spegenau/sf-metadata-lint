use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailTemplateSettings  {
	#[serde(rename = "enableTemplateEnhancedFolderPref")]
	pub enable_template_enhanced_folder_pref: Option<bool>,
}
