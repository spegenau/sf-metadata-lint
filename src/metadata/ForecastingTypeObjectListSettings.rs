use crate::metadata::ForecastingObjectListLabelMapping::ForecastingObjectListLabelMapping;
use crate::metadata::ForecastingObjectListSelectedSettings::ForecastingObjectListSelectedSettings;
use crate::metadata::ForecastingObjectListUnselectedSettings::ForecastingObjectListUnselectedSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingTypeObjectListSettings  {
	#[serde(rename = "forecastingObjectListLabelMappings")]
	pub forecasting_object_list_label_mappings: Option<Vec<ForecastingObjectListLabelMapping>>,
	#[serde(rename = "forecastingObjectListSelectedSettings")]
	pub forecasting_object_list_selected_settings: ForecastingObjectListSelectedSettings,
	#[serde(rename = "forecastingObjectListUnselectedSettings")]
	pub forecasting_object_list_unselected_settings: ForecastingObjectListUnselectedSettings,
	#[serde(rename = "forecastingTypeDeveloperName")]
	pub forecasting_type_developer_name: String,
}
