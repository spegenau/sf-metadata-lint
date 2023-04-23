use crate::metadata::ComponentInstancePropertyListItem::ComponentInstancePropertyListItem;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ComponentInstancePropertyList  {
	#[serde(rename = "valueListItems")]
	pub value_list_items: Option<Vec<ComponentInstancePropertyListItem>>,
}
