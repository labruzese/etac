use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use crate::flags;
use crate::lexer;
use crate::sources::{EtaSpan, FileId, SourceManager};

#[derive(Debug)]
pub enum ParseError {
    IOError(std::io::Error),
    UnknownSource,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::IOError(err)
    }
}

// Update signature to take file_id and the manager
pub fn parse(sm: SourceManager, file_id: FileId) -> Result<(), ParseError> {
    let options = flags::flags();
    let source = sm.get_source(file_id).ok_or(ParseError::UnknownSource)?;
    let lexer = lexer::Lexer::new(&source).spanned();

    // make some writer for verbose lexing if flag is set
    let (mut writer, location_resolver) = if options.lex {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&options.lex_file)?;

        (
            Some(BufWriter::new(file)),
            Some(ariadne::Source::from(&source)),
        )
    } else {
        (None, None)
    };

    // iterate tokens
    for (token_result, span) in lexer {
        match token_result {
            // Successful token
            Ok(token) => {
                // if we have a writer write our lexing output
                if let Some(w) = &mut writer
                    && let Some(lresolver) = &location_resolver
                {
                    let (_, line, col) = lresolver
                        .get_byte_line(span.start)
                        .expect("couldn't resolve location from byte offset");
                    writeln!(w, "{}:{} {}", line, col, token)?;
                }

                // TODO: actual parsing logic
            }
            // diagnostic produced by lexer
            Err(mut diag) => {
                let eta_span: EtaSpan = (file_id, span).into();
                // We modify the existing Diagnostic to add the label
                // Since this comes from the Lexer (e.g., bad int), we flag the specific text.
                diag = diag.with_primary_label(eta_span.clone(), "Invalid token");

                // Emit the error
                sm.emit(diag, eta_span);
            }
        }
    }

    if let Some(mut w) = writer {
        w.flush()?;
    }

    Ok(())
}
