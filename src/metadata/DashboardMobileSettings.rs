use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DashboardMobileSettings  {
	#[serde(rename = "enableDashboardIPadApp")]
	pub enable_dashboard_i_pad_app: Option<bool>,
}
