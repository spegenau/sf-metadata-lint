use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct BlockchainSettings  {
	#[serde(rename = "enableBcp")]
	pub enable_bcp: Option<bool>,
	#[serde(rename = "enableEtpNft")]
	pub enable_etp_nft: Option<bool>,
}
