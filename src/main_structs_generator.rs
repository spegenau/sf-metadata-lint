pub mod utilities;
pub use utilities::*;

pub use wsdl_structs::*;

fn main() {
    wsdl::WSDL::generate_structs();
}


