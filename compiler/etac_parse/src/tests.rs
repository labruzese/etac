#[cfg(test)]
mod tests {
    use crate::grammar;
    
    // helpers for setting up file id and cache and everything and the parse_test! macro
        
    #[test]
    fn no_definitions() {
        let prog = r#"use io"#;
        let parse_result = parse_test!(prog);
    }
}
