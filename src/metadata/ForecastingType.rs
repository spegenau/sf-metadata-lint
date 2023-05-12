use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingType  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "amount")]
	pub amount: bool,
	#[serde(rename = "dateType")]
	pub date_type: String,
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "hasProductFamily")]
	pub has_product_family: bool,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "opportunitySplitType")]
	pub opportunity_split_type: Option<String>,
	#[serde(rename = "quantity")]
	pub quantity: bool,
	#[serde(rename = "roleType")]
	pub role_type: String,
	#[serde(rename = "territory2Model")]
	pub territory_2_model: Option<String>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
