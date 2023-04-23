use crate::metadata::FlowActionCallInputParameter::FlowActionCallInputParameter;
use crate::metadata::FlowActionCallOutputParameter::FlowActionCallOutputParameter;
use crate::metadata::FlowConnector::FlowConnector;
use crate::metadata::FlowDataTypeMapping::FlowDataTypeMapping;
use crate::metadata::FlowTransactionModel::FlowTransactionModel;
use crate::metadata::InvocableActionType::InvocableActionType;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FlowActionCall  {
	#[serde(rename = "actionName")]
	pub action_name: String,
	#[serde(rename = "actionType")]
	pub action_type: InvocableActionType,
	#[serde(rename = "connector")]
	pub connector: Option<FlowConnector>,
	#[serde(rename = "dataTypeMappings")]
	pub data_type_mappings: Option<Vec<FlowDataTypeMapping>>,
	#[serde(rename = "faultConnector")]
	pub fault_connector: Option<FlowConnector>,
	#[serde(rename = "flowTransactionModel")]
	pub flow_transaction_model: Option<FlowTransactionModel>,
	#[serde(rename = "inputParameters")]
	pub input_parameters: Option<Vec<FlowActionCallInputParameter>>,
	#[serde(rename = "outputParameters")]
	pub output_parameters: Option<Vec<FlowActionCallOutputParameter>>,
	#[serde(rename = "storeOutputAutomatically")]
	pub store_output_automatically: Option<bool>,
}
