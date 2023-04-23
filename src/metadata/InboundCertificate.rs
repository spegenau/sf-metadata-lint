use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct InboundCertificate  {
	#[serde(rename = "expirationDate")]
	pub expiration_date: String,
	#[serde(rename = "issuer")]
	pub issuer: String,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "serialId")]
	pub serial_id: String,
}
