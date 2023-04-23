use crate::metadata::LwcResource::LwcResource;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct LwcResources  {
	#[serde(rename = "lwcResource")]
	pub lwc_resource: Option<Vec<LwcResource>>,
}
