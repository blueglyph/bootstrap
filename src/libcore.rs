// ---------------------------------------------------------------------------------------------
// Represents the library

/// Library version
pub const LIB_VERSION: &str = "lib 1";

pub trait Parse {
    fn get_lib(&self) -> String;
    fn get_gen(&self) -> String;
    fn get_grammar(&self) -> String;
}
