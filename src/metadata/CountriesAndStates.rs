use crate::metadata::Country::Country;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CountriesAndStates  {
	#[serde(rename = "countries")]
	pub countries: Option<Vec<Country>>,
}
