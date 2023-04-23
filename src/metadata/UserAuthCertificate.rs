use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct UserAuthCertificate  {
	#[serde(rename = "developerName")]
	pub developer_name: String,
	#[serde(rename = "expirationDate")]
	pub expiration_date: Option<String>,
	#[serde(rename = "masterLabel")]
	pub master_label: String,
	#[serde(rename = "serialNumber")]
	pub serial_number: String,
	#[serde(rename = "user")]
	pub user: String,
}
