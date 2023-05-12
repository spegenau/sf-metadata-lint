use crate::metadata::ExternalServiceOperation::ExternalServiceOperation;
use crate::metadata::ExternalServiceRegistrationProviderType::ExternalServiceRegistrationProviderType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ExternalServiceRegistration  {
	#[serde(rename = "description")]
	pub description: Option<String>,
	#[serde(rename = "label")]
	pub label: String,
	#[serde(rename = "namedCredential")]
	pub named_credential: Option<String>,
	#[serde(rename = "namedCredentialReference")]
	pub named_credential_reference: Option<String>,
	#[serde(rename = "operations")]
	pub operations: Option<Vec<ExternalServiceOperation>>,
	#[serde(rename = "providerAssetEndpoint")]
	pub provider_asset_endpoint: Option<String>,
	#[serde(rename = "registrationProviderType")]
	pub registration_provider_type: Option<ExternalServiceRegistrationProviderType>,
	#[serde(rename = "schema")]
	pub schema: Option<String>,
	#[serde(rename = "schemaAbsoluteUrl")]
	pub schema_absolute_url: Option<String>,
	#[serde(rename = "schemaType")]
	pub schema_type: Option<String>,
	#[serde(rename = "schemaUploadFileExtension")]
	pub schema_upload_file_extension: Option<String>,
	#[serde(rename = "schemaUploadFileName")]
	pub schema_upload_file_name: Option<String>,
	#[serde(rename = "schemaUrl")]
	pub schema_url: Option<String>,
	#[serde(rename = "serviceBinding")]
	pub service_binding: Option<String>,
	#[serde(rename = "status")]
	pub status: String,
	#[serde(rename = "systemVersion")]
	pub system_version: Option<i32>,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
