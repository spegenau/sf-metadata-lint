use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PicklistSettings  {
	#[serde(rename = "isPicklistApiNameEditDisabled")]
	pub is_picklist_api_name_edit_disabled: Option<bool>,
}
