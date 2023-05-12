use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CollectionsDashboardSettings  {
	#[serde(rename = "enableCollectionsDashboard")]
	pub enable_collections_dashboard: Option<bool>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
