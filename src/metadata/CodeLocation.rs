use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CodeLocation  {
	#[serde(rename = "column")]
	pub column: i32,
	#[serde(rename = "line")]
	pub line: i32,
	#[serde(rename = "numExecutions")]
	pub num_executions: i32,
	#[serde(rename = "time")]
	pub time: f32,
}
