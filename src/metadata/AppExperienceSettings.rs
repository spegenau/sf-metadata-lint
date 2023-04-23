use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppExperienceSettings  {
	#[serde(rename = "doesHideAllAppsInAppLauncher")]
	pub does_hide_all_apps_in_app_launcher: Option<bool>,
}
