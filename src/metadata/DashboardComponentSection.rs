use crate::metadata::DashboardComponent::DashboardComponent;
use crate::metadata::DashboardComponentSize::DashboardComponentSize;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardComponentSection  {
	#[serde(rename = "columnSize")]
	pub column_size: DashboardComponentSize,
	#[serde(rename = "components")]
	pub components: Option<Vec<DashboardComponent>>,
}
