// src/logger.rs
use crate::ast;
use crate::cli::Flags;
use crate::sources::{EtaSpan, FileId, Sources};
use ariadne::Cache;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

pub struct Logger {
    lexer_writers: Option<HashMap<FileId, BufWriter<File>>>,
    parser_writers: Option<HashMap<FileId, BufWriter<File>>>,
    diag_root: PathBuf,
}

impl Logger {
    pub fn new(flags: &Flags) -> Self {
        if flags.lex || flags.parse {
            std::fs::create_dir_all(&flags.diag_path)
                .expect("unable to create diagnostic output directory");
        }
        Self {
            lexer_writers: flags.lex.then(HashMap::new),
            parser_writers: flags.parse.then(HashMap::new),
            diag_root: flags.diag_path.clone(),
        }
    }

    pub fn log_token(
        &mut self,
        sources: &mut Sources,
        at: EtaSpan,
        token: &impl std::fmt::Display,
    ) {
        let Some(writers) = self.lexer_writers.as_mut() else {
            return;
        };
        let (line, col) = line_col(sources, &at);
        let diag_root = &self.diag_root;
        let w = writers
            .entry(at.file_id.clone())
            .or_insert_with(|| open_log(diag_root, at.file_id.as_str(), "lexed"));
        writeln!(w, "{}:{} {}", line, col, token).unwrap();
    }

    pub fn log_lexical_error(&mut self, sources: &mut Sources, at: EtaSpan, message: &str) {
        let Some(writers) = self.lexer_writers.as_mut() else {
            return;
        };
        let (line, col) = line_col(sources, &at);
        let diag_root = &self.diag_root;
        let w = writers
            .entry(at.file_id.clone())
            .or_insert_with(|| open_log(diag_root, at.file_id.as_str(), "lexed"));
        writeln!(w, "{}:{} error:{}", line, col, message).unwrap();
        self.lexer_writers = None; // detach after first report
    }

    // log_parse doesn't need line/col so no Sources needed
    pub fn log_parse(&mut self, file_id: &FileId, program: &ast::Program) {
        let Some(writers) = self.parser_writers.as_mut() else {
            return;
        };
        let diag_root = &self.diag_root;
        let w = writers
            .entry(file_id.clone())
            .or_insert_with(|| open_log(diag_root, file_id.as_str(), "parsed"));
        writeln!(w, "{}", program).unwrap();
    }

    pub fn log_syntactic_error(&mut self, sources: &mut Sources, at: EtaSpan, message: &str) {
        let Some(writers) = self.parser_writers.as_mut() else {
            return;
        };
        let (line, col) = line_col(sources, &at);
        let diag_root = &self.diag_root;
        let w = writers
            .entry(at.file_id.clone())
            .or_insert_with(|| open_log(diag_root, at.file_id.as_str(), "parsed"));
        writeln!(w, "{}:{} error:{}", line, col, message).unwrap();
        self.parser_writers = None;
    }

    pub fn flush(&mut self) {
        if let Some(m) = &mut self.lexer_writers {
            for w in m.values_mut() {
                let _ = w.flush();
            }
        }
        if let Some(m) = &mut self.parser_writers {
            for w in m.values_mut() {
                let _ = w.flush();
            }
        }
    }
}

fn line_col(sources: &mut Sources, at: &EtaSpan) -> (usize, usize) {
    let src = sources.fetch(&at.file_id).expect("missing file");
    let (_, l, c) = src.get_byte_line(at.range.start).expect("bad byte offset");
    (l + 1, c + 1)
}

fn open_log(root: &Path, file_name: &str, ext: &str) -> BufWriter<File> {
    let path = root.join(file_name).with_extension(ext);
    BufWriter::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("unable to open diagnostic file"),
    )
}
