pub mod utilities;
pub use utilities::*;

fn main() {
    wsdl::WSDL::generate_structs();
}


