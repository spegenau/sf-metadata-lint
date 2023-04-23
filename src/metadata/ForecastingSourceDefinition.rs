use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingSourceDefinition  {
	#[serde(rename = "categoryField")]
	pub category_field: Option<String>,
	#[serde(rename = "dateField")]
	pub date_field: Option<String>,
	#[serde(rename = "familyField")]
	pub family_field: Option<String>,
	#[serde(rename = "isProtected")]
	pub is_protected: Option<bool>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "measureField")]
	pub measure_field: Option<String>,
	#[serde(rename = "sourceObject")]
	pub source_object: String,
	#[serde(rename = "territory2Field")]
	pub territory_2_field: Option<String>,
	#[serde(rename = "userField")]
	pub user_field: Option<String>,
}
