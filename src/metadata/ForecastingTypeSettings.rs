use crate::metadata::ForecastingDateType::ForecastingDateType;
use crate::metadata::OpportunityListFieldsLabelMapping::OpportunityListFieldsLabelMapping;
use crate::metadata::OpportunityListFieldsSelectedSettings::OpportunityListFieldsSelectedSettings;
use crate::metadata::OpportunityListFieldsUnselectedSettings::OpportunityListFieldsUnselectedSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingTypeSettings  {
	#[serde(rename = "active")]
	pub active: bool,
	#[serde(rename = "displayedCategoryApiNames")]
	pub displayed_category_api_names: Option<Vec<String>>,
	#[serde(rename = "forecastedCategoryApiNames")]
	pub forecasted_category_api_names: Option<Vec<String>>,
	#[serde(rename = "forecastingDateType")]
	pub forecasting_date_type: ForecastingDateType,
	#[serde(rename = "hasProductFamily")]
	pub has_product_family: bool,
	#[serde(rename = "isAmount")]
	pub is_amount: bool,
	#[serde(rename = "isAvailable")]
	pub is_available: bool,
	#[serde(rename = "isQuantity")]
	pub is_quantity: bool,
	#[serde(rename = "managerAdjustableCategoryApiNames")]
	pub manager_adjustable_category_api_names: Option<Vec<String>>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "name")]
	pub name: String,
	#[serde(rename = "opportunityListFieldsLabelMappings")]
	pub opportunity_list_fields_label_mappings: Option<Vec<OpportunityListFieldsLabelMapping>>,
	#[serde(rename = "opportunityListFieldsSelectedSettings")]
	pub opportunity_list_fields_selected_settings: OpportunityListFieldsSelectedSettings,
	#[serde(rename = "opportunityListFieldsUnselectedSettings")]
	pub opportunity_list_fields_unselected_settings: OpportunityListFieldsUnselectedSettings,
	#[serde(rename = "opportunitySplitName")]
	pub opportunity_split_name: Option<String>,
	#[serde(rename = "ownerAdjustableCategoryApiNames")]
	pub owner_adjustable_category_api_names: Option<Vec<String>>,
	#[serde(rename = "territory2ModelName")]
	pub territory_2_model_name: Option<String>,
}
