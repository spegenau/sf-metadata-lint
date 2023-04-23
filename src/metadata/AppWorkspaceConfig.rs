use crate::metadata::WorkspaceMapping::WorkspaceMapping;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppWorkspaceConfig  {
	#[serde(rename = "mappings")]
	pub mappings: Option<Vec<WorkspaceMapping>>,
}
