use crate::metadata::ObjectSearchSetting::ObjectSearchSetting;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct SearchSettingsByObject  {
	#[serde(rename = "searchSettingsByObject")]
	pub search_settings_by_object: Option<Vec<ObjectSearchSetting>>,
}
