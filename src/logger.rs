use ariadne::Cache;

use crate::{ast, cli};
use crate::sources::{EtaSpan, FileId};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::{PathBuf};

/// Handles printing the lexer log if enabled
pub struct Logger {
    lexer_writer: Option<HashMap<FileId, BufWriter<std::fs::File>>>,
    parser_writer: Option<HashMap<FileId, BufWriter<std::fs::File>>>,
    diag_root: PathBuf,
}

impl<'source_cache> Logger {
    pub fn new(options: &cli::Flags) -> Self {
        let mut me = Self { 
            lexer_writer:  None,
            parser_writer:  None,
            diag_root: options.diag_path.clone(),
        };
        if options.lex || options.parse {
            let _ = me.resolver.insert(sources);
            std::fs::create_dir_all(&options.diag_path)
                .expect("unable to create diagnostic output directory");
        }
        if options.lex {
            me.lexer_writer = Some(HashMap::new());
        }    
        if options.parse {
            me.parser_writer = Some(HashMap::new());
        }    
        me
    }

    pub fn is_logging_lexer(&self) -> bool { self.lexer_writer.is_some() }
    pub fn is_logging_parser(&self) -> bool { self.parser_writer.is_some() }

    pub fn log_token(&mut self, at: EtaSpan, token: &impl std::fmt::Display) {
        if let Some(writers) = &mut self.lexer_writer
        {
            let w = writers.entry(at.file_id.clone()).or_insert_with(|| {
                let mut path = self.diag_root.clone();
                path.push(at.file_id.as_str());
                path.with_extension("lexed");
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .expect("unable to open parse file to write into"))
            });
            //these are 0 indexed
            let (_, line, col) = {
                self.resolver
                    .as_ref()
                    .unwrap()
                    .lock()
                    .expect("lock poisoned")
                    .fetch(&at.file_id)
                    .expect("couldn't find file")
                    .get_byte_line(at.range.start)
                    .expect("couldn't load byte address")
            };
            writeln!(w, "{}:{} {}", line + 1, col + 1, token)
                .expect("failed to write to lex file buffer");
        }
    }

    pub fn log_parse(&mut self, file_id: &FileId, program: &ast::Program) {
        if let Some(writers) = &mut self.parser_writer {
            let w = writers.entry(file_id.clone()).or_insert_with(|| {
                let mut path = self.diag_root.clone();
                path.push(file_id.as_str());
                path.with_extension("parsed");
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .expect("unable to open parse file to write into"))
            });
            writeln!(w, "{}", program)
                .expect("failed to write to parse file buffer");
        }
    }

    pub fn log_lexical_error(&mut self, at: EtaSpan, message: &str) {
        if let Some(writers) = &mut self.lexer_writer
        {
            let w = writers.entry(at.file_id.clone()).or_insert_with(|| {
                let mut path = self.diag_root.clone();
                path.push(at.file_id.as_str());
                path.with_extension("lexed");
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .expect("unable to open parse file to write into"))
            });
            let (_, line, col) = {
                self.resolver
                    .as_ref()
                    .unwrap()
                    .lock()
                    .expect("lock poisoned")
                    .fetch(&at.file_id)
                    .expect("couldn't find file")
                    .get_byte_line(at.range.start)
                    .expect("couldn't load byte address")
            };
            writeln!(w, "{}:{} error:{}", line + 1, col + 1, message)
                .expect("failed to write to lex file buffer");
            //detach after first report
            self.lexer_writer = None;
        }
    }

    pub fn log_syntatic_error(&mut self, at: EtaSpan, message: &str) {
        if let Some(writers) = &mut self.parser_writer
        {
            let w = writers.entry(at.file_id.clone()).or_insert_with(|| {
                let mut path = self.diag_root.clone();
                path.push(at.file_id.as_str());
                path.with_extension("parsed");
                BufWriter::new(OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)
                    .expect("unable to open parse file to write into"))
            });
            let (_, line, col) = {
                self.resolver
                    .as_ref()
                    .unwrap()
                    .lock()
                    .expect("lock poisoned")
                    .fetch(&at.file_id)
                    .expect("couldn't find file")
                    .get_byte_line(at.range.start)
                    .expect("couldn't load byte address")
            };
            writeln!(w, "{}:{} error:{}", line + 1, col + 1, message)
                .expect("failed to write to parse file buffer");
            //detach after first report
            self.parser_writer = None;
        }
    }

    pub fn flush(&mut self) {
        if let Some(w) = &mut self.lexer_writer {
            w.flush().expect("failed to flush writer to lex file");
        }
        if let Some(w) = &mut self.parser_writer {
            w.flush().expect("failed to flush writer to parse file");
        }
    }
}
