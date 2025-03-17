#[cfg(test)]
mod base_test {
    use lexiparser_bootstrap::libcore::Parse;
    use crate::lexiparser::build_parser;

    #[test]
    fn test_parser() {
        let parser = build_parser();
        assert_eq!(parser.get_lib(), "lib 3.0");
        assert_eq!(parser.get_gen(), "gen 2.0");
        assert_eq!(parser.get_grammar(), "grammar 1");
    }
}
