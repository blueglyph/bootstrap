mod out;

#[cfg(test)]
mod base_test {
    use crate::out::lexiparser::lexiparser::build_parser;
    use bootstrap::libcore::Parse;

    #[test]
    fn test_parser() {
        let parser = build_parser();
        assert_eq!(parser.get_lib(), "lib 1");
        assert_eq!(parser.get_gen(), "gen 1");
        assert_eq!(parser.get_grammar(), "grammar 1");
    }
}