use std::rc::Rc;
use std::sync::Mutex;

use crate::{ast, source, LOGGER};
use crate::errors::{Diagnostic, NoFileDiagnostic};
use crate::lexer;
use crate::logger;
use crate::lexer::Token;
use crate::logger::*;
use crate::sources::{FileId, Sources};
use crate::{cli, error};
use ariadne::Cache;
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(grammar);

// Update signature to take file_id and the manager
pub fn parse(file_id: FileId) -> Result<ast::Program, Vec<Diagnostic>> {
    let source = source!(file_id);
    let lexer = lexer::Lexer::new(source.text()).spanned();

    // lex (unfortunately we can't stream the lexer since we need to complete lexing even if parsing
    // fails)
    let tokens: Vec<Result<(usize, Token, usize), NoFileDiagnostic>> = lexer
        .map(|(tok, span)| match tok {
            Ok(t) => {
                logger!(|l| l.log_token((&file_id, span.clone()).into(), &t));
                Ok((span.start, t, span.end))
            }
            Err(d) => {
                logger!(|l| l.log_lexical_error((&file_id, span).into(), &d.message));
                Err(d)
            }
        })
    .collect();

    // parse
    let mut recovered: Vec<lalrpop_util::ErrorRecovery<usize, Token, NoFileDiagnostic>> =
        Vec::new();
    let result = grammar::ProgramParser::new().parse(&mut recovered, tokens);

    if result.is_err() || !recovered.is_empty() {
        let mut errors: Vec<Diagnostic> = recovered
            .into_iter()
            .map(|r| to_diag(r.error).specify_file(&file_id))
            .collect();

        if let Err(e) = result {
            errors.push(to_diag(e).specify_file(&file_id));
        }

        for err in errors.iter() {
            logger!(|l| l.log_syntatic_error(err.loc, &err.message));
        }

        return Err(errors);
    }

    let program = result.unwrap();
    logger!(|l| l.log_parse(&file_id, &program));
    Ok(program)
}

fn to_diag(err: ParseError<usize, Token, NoFileDiagnostic>) -> NoFileDiagnostic {
    use ParseError::*;
    match err {
        User { error } => error,

        UnrecognizedToken {
            token: (s, t, e),
            expected,
        } => error!(s..e, "Unexpected token {}", t).with_primary_label(format_expected(&expected)),

        UnrecognizedEof { location, expected } => {
            error!(location..location, "Unexpected end of file")
                .with_primary_label(format_expected(&expected))
        }

        ExtraToken { token: (s, t, e) } => {
            error!(s..e, "Extra token {} after program", t).with_primary_label("unexpected")
        }

        InvalidToken { location: _ } => {
            unreachable!("external lexer; lalrpop can not recieve an invalid token")
        }
    }
}

fn format_expected(expected: &[String]) -> String {
    match expected.len() {
        0 => "expected nothing".into(),
        1 => format!("expected {}", expected[0]),
        _ => {
            let (last, rest) = expected.split_last().unwrap();
            format!("expected one of {}, or {}", rest.join(", "), last)
        }
    }
}
