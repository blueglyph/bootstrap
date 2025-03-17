// -------------------------------------------------------------------------
// [lexiparser]

pub use lexiparser_bootstrap::libcore::Parse;

pub struct MyParser {}

pub fn build_parser() -> impl Parse {
    MyParser { }
}

impl Parse for MyParser {
    fn get_lib(&self) -> String {
        "lib 1".to_string()
    }

    fn get_gen(&self) -> String {
        "gen 2.0".to_string()
    }

    fn get_grammar(&self) -> String {
        "grammar 1".to_string()
    }
}

// [lexiparser]
// -------------------------------------------------------------------------
