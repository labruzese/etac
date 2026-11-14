//! Loading files into the source map, and turning offsets back into locations.

use std::sync::atomic::{AtomicUsize, Ordering};

use etac_source::{SourceMap, Span};

/// The offset of the start of each line of `text`, going by the line the map says each character is on.
fn line_starts(text: &str) -> Vec<usize> {
    let sm = SourceMap::default();
    let (id, _) = sm.alloc_source("lines.eta".to_owned(), text.to_owned());
    let base = sm.base_offset(id);

    let mut starts = vec![0];
    let mut line = 1;
    for (offset, _) in text.char_indices() {
        let at = base + offset as u32;
        let (line_here, _) = sm.span_location(Span::new(at, at)).start;
        if line_here != line {
            starts.push(offset);
            line = line_here;
        }
    }
    starts
}

#[test]
fn a_file_is_loaded_once_however_many_threads_ask() {
    let sm = SourceMap::default();
    let reads = AtomicUsize::new(0);
    let ids = std::thread::scope(|threads| {
        let workers: Vec<_> = (0..16)
            .map(|_| {
                threads.spawn(|| {
                    sm.get_or_load("a.eta", || {
                        reads.fetch_add(1, Ordering::SeqCst);
                        Ok::<_, std::convert::Infallible>("f() { }".to_owned())
                    })
                    .unwrap()
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).collect::<Vec<_>>()
    });

    assert_eq!(reads.load(Ordering::SeqCst), 1);
    assert!(ids.iter().all(|id| *id == ids[0]));
    assert_eq!(&*sm.get_source(ids[0]).source, "f() { }");
}

#[test]
fn lines_break_on_every_terminator() {
    assert_eq!(line_starts(""), [0]);
    assert_eq!(line_starts("a"), [0]);
    assert_eq!(line_starts("a\n"), [0]);
    assert_eq!(line_starts("a\nb"), [0, 2]);
    assert_eq!(line_starts("a\r\nb\rc\n\nd"), [0, 3, 5, 7, 8]);
    assert_eq!(
        line_starts("a\x0Bb\x0Cc\u{85}d\u{2028}e\u{2029}f"),
        [0, 2, 4, 7, 11, 15]
    );
}

#[test]
fn locations_are_one_based() {
    let sm = SourceMap::default();
    let (_, _) = sm.alloc_source("skip.eta".to_owned(), "xx".to_owned());
    let (id, text) = sm.alloc_source("a.eta".to_owned(), "ab\ncd\n".to_owned());
    let base = sm.base_offset(id);
    let at = |lo: usize, hi: usize| {
        sm.span_location(Span::new(base + lo as u32, base + hi as u32))
            .to_string()
    };
    assert_eq!(at(0, 2), "a.eta:1:1..1:3");
    assert_eq!(at(3, 5), "a.eta:2:1..2:3");
    // a terminator at the very end of the text does not start another line
    assert_eq!(at(text.len(), text.len()), "a.eta:2:4..2:4");
}

#[test]
fn columns_count_characters() {
    let sm = SourceMap::default();
    // `é` is 2 bytes and `→` is 3
    let (id, text) = sm.alloc_source("a.eta".to_owned(), "é→x\n→y".to_owned());
    let base = sm.base_offset(id);
    let at = |needle: char| {
        let lo = text.find(needle).unwrap();
        let span = Span::new(base + lo as u32, base + (lo + needle.len_utf8()) as u32);
        sm.span_location(span).to_string()
    };
    assert_eq!(at('é'), "a.eta:1:1..1:2");
    assert_eq!(at('x'), "a.eta:1:3..1:4");
    assert_eq!(at('y'), "a.eta:2:2..2:3");
}

#[test]
fn a_failed_load_stores_nothing() {
    let sm = SourceMap::default();
    assert_eq!(sm.get_or_load("a.eta", || Err("unreadable")), Err("unreadable"));
    assert!(sm.get_id("a.eta").is_none());
    let id = sm.get_or_load("a.eta", || Ok::<_, ()>(String::new())).unwrap();
    assert_eq!(sm.get_id("a.eta"), Some(id));
}
