use crate::metadata::ContentAssetVersion::ContentAssetVersion;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ContentAssetVersions  {
	#[serde(rename = "version")]
	pub version: Option<Vec<ContentAssetVersion>>,
}
