use crate::metadata::SamlEncryptionType::SamlEncryptionType;
use crate::metadata::SamlIdpSLOBinding::SamlIdpSLOBinding;
use crate::metadata::SamlNameIdFormatType::SamlNameIdFormatType;
use crate::metadata::SamlSigningAlgoType::SamlSigningAlgoType;
use crate::metadata::SamlSubjectType::SamlSubjectType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct ConnectedAppSamlConfig  {
	#[serde(rename = "acsUrl")]
	pub acs_url: String,
	#[serde(rename = "certificate")]
	pub certificate: Option<String>,
	#[serde(rename = "encryptionCertificate")]
	pub encryption_certificate: Option<String>,
	#[serde(rename = "encryptionType")]
	pub encryption_type: Option<SamlEncryptionType>,
	#[serde(rename = "entityUrl")]
	pub entity_url: String,
	#[serde(rename = "issuer")]
	pub issuer: Option<String>,
	#[serde(rename = "samlIdpSLOBindingEnum")]
	pub saml_idp_slo_binding_enum: Option<SamlIdpSLOBinding>,
	#[serde(rename = "samlNameIdFormat")]
	pub saml_name_id_format: Option<SamlNameIdFormatType>,
	#[serde(rename = "samlSigningAlgoType")]
	pub saml_signing_algo_type: Option<SamlSigningAlgoType>,
	#[serde(rename = "samlSloUrl")]
	pub saml_slo_url: Option<String>,
	#[serde(rename = "samlSubjectCustomAttr")]
	pub saml_subject_custom_attr: Option<String>,
	#[serde(rename = "samlSubjectType")]
	pub saml_subject_type: SamlSubjectType,
}
