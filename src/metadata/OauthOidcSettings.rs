use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct OauthOidcSettings  {
	#[serde(rename = "blockOAuthUnPwFlow")]
	pub block_o_auth_un_pw_flow: Option<bool>,
	#[serde(rename = "blockOAuthUsrAgtFlow")]
	pub block_o_auth_usr_agt_flow: Option<bool>,
	#[serde(rename = "enableHdlessFgtPswFlow")]
	pub enable_hdless_fgt_psw_flow: Option<bool>,
	#[serde(rename = "oAuthCdCrdtFlowEnable")]
	pub o_auth_cd_crdt_flow_enable: Option<bool>,
}
