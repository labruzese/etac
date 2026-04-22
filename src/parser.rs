use crate::ast;
use crate::context::Context;
use crate::error;
use crate::errors::{Diagnostic, NoFileDiagnostic};
use crate::lexer;
use crate::lexer::Token;
use crate::sources::FileId;
use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(grammar);

// src/parser.rs
pub fn parse(ctx: &mut Context, file_id: &FileId) -> Result<ast::Program, Vec<Diagnostic>> {
    let source = ctx
        .sources
        .text(file_id);

    let lexer = lexer::Lexer::new(&source).spanned();
    let tokens: Vec<Result<(usize, Token, usize), NoFileDiagnostic>> = lexer
        .map(|(tok, span)| match tok {
            Ok(t) => {
                ctx.logger
                    .log_token(&mut ctx.sources, (file_id, span.clone()).into(), &t);
                Ok((span.start, t, span.end))
            }
            Err(d) => {
                ctx.logger
                    .log_lexical_error(&mut ctx.sources, (file_id, span).into(), &d.message);
                Err(d)
            }
        })
        .collect();

    let mut recovered = Vec::new();
    let result = grammar::ProgramParser::new().parse(&mut recovered, tokens);

    if result.is_err() || !recovered.is_empty() {
        let mut errors: Vec<Diagnostic> = recovered
            .into_iter()
            .map(|r| to_diag(r.error).specify_file(file_id))
            .collect();
        if let Err(e) = result {
            errors.push(to_diag(e).specify_file(file_id));
        }
        for err in &errors {
            ctx.logger
                .log_syntactic_error(&mut ctx.sources, err.loc.clone(), &err.message);
        }
        return Err(errors);
    }

    let program = result.unwrap();
    ctx.logger.log_parse(file_id, &program);
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
