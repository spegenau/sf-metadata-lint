use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct PermissionSetExternalDataSourceAccess  {
	#[serde(rename = "enabled")]
	pub enabled: bool,
	#[serde(rename = "externalDataSource")]
	pub external_data_source: String,
}
