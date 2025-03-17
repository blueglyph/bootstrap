use bootstrap::libgen::indent_source;
use lexiparser::lexiparser::build_parser;   // crate parser
use lexiparser::lexiparser::Parse;          // we must use the crate's trait

/// Generator version
const GEN_VERSION: &str = "gen 2.0";

/// Creates the custom parser
pub fn build_parser_manual() -> (String, String, String) {
    let parser = build_parser();
    (
        parser.get_lib(),
        GEN_VERSION.to_string(),
        parser.get_grammar()
    )
}

/// Writes the parser source code
pub fn write_source_code(name: String) -> String {
    let (lib, generator, grammar) = build_parser_manual();
    let mut src1 = Vec::<String>::new();
    src1.push("use bootstrap::libcore::Parse;".to_string());
    let mut src2 = Vec::<String>::new();
    src2.push("pub struct MyParser {}".to_string());
    src2.push("".to_string());
    src2.push("pub fn build_parser() -> impl Parse {".to_string());
    src2.push(format!("    {name} {{ }}"));
    src2.push("}".to_string());
    src2.push("".to_string());
    src2.push("impl Parse for MyParser {".to_string());
    src2.push("    fn get_lib(&self) -> String {".to_string());
    src2.push(format!("        \"{lib}\".to_string()"));
    src2.push("    }".to_string());
    src2.push("".to_string());
    src2.push("    fn get_gen(&self) -> String {".to_string());
    src2.push(format!("        \"{generator}\".to_string()"));
    src2.push("    }".to_string());
    src2.push("".to_string());
    src2.push("    fn get_grammar(&self) -> String {".to_string());
    src2.push(format!("        \"{grammar}\".to_string()"));
    src2.push("    }".to_string());
    src2.push("}".to_string());
    indent_source(vec![src1, src2], 4)
}

#[cfg(test)]
mod tests {
    use bootstrap::libgen::{get_tagged_source, replace_tagged_source};
    use super::*;

    const FILENAME: &str = "lexiparser/src/lexiparser.rs";
    const TAG: &str = "lexiparser";

    fn build_sources() -> String {
        write_source_code("MyParser".to_string())
    }

    #[test]
    /// Verifies the source code (must have been generated before)
    fn verify_parser() {
        let src = build_sources();
        assert_eq!(get_tagged_source(FILENAME, TAG), Some(src));
    }

    #[test]
    #[ignore]
    /// Launch this test to generate the source code
    fn write_parser() {
        let src = build_sources();
        replace_tagged_source(FILENAME, TAG, &src).expect("Couldn't write the source file");
    }
}