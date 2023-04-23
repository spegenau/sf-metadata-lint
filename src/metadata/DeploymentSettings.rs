use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct DeploymentSettings  {
	#[serde(rename = "doesSkipAsyncApexValidation")]
	pub does_skip_async_apex_validation: Option<bool>,
}
