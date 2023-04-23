use crate::metadata::ForecastingTypeObjectListSettings::ForecastingTypeObjectListSettings;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ForecastingObjectListSettings  {
	#[serde(rename = "forecastingTypeObjectListSettings")]
	pub forecasting_type_object_list_settings: Option<Vec<ForecastingTypeObjectListSettings>>,
}
