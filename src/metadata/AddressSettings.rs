use crate::metadata::CountriesAndStates::CountriesAndStates;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AddressSettings  {
	#[serde(rename = "countriesAndStates")]
	pub countries_and_states: CountriesAndStates,
	#[serde(rename = "fullName")]
	pub full_name: Option<String>,
}
