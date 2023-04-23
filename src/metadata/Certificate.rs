use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Certificate  {
	#[serde(rename = "caSigned")]
	pub ca_signed: bool,
	#[serde(rename = "encryptedWithPlatformEncryption")]
	pub encrypted_with_platform_encryption: Option<bool>,
	#[serde(rename = "expirationDate")]
	pub expiration_date: Option<String>,
	#[serde(rename = "keySize")]
	pub key_size: Option<i32>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "privateKeyExportable")]
	pub private_key_exportable: Option<bool>,
}
