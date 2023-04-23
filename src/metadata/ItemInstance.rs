use crate::metadata::ComponentInstance::ComponentInstance;
use crate::metadata::FieldInstance::FieldInstance;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ItemInstance  {
	#[serde(rename = "componentInstance")]
	pub component_instance: Option<ComponentInstance>,
	#[serde(rename = "fieldInstance")]
	pub field_instance: Option<FieldInstance>,
}
