use crate::internal_error::{InternalLexerError, lexer_error};

use super::{current_span, LogosLexer, Span};

const VALID_ESCAPES: &str = "valid escapes: '\\n', '\\t', '\\r', '\\\\', '\\'', '\\\"', '\\0', '\\x{..}'";

struct Cursor<'a> {
    input: &'a [u8],
    base: u32, // in global space
    pos: u32, // in global space
}

impl<'a> Cursor<'a> {
    fn new(s: &'a str, base: u32) -> Self {
        Self {
            input: s.as_bytes(),
            base,
            pos: base,
        }
    }
    fn peek(&self) -> Option<u8> {
        self.input.get((self.pos - self.base) as usize).copied()
    }
    fn next(&mut self) -> Option<u8> {
        let b = self.input.get((self.pos - self.base) as usize).copied();
        if b.is_some() {
            self.pos += 1;
        }
        b
    }
    /// `[start, self.pos)`.
    fn slice_from(&self, start: u32) -> &'a str {
        std::str::from_utf8(&self.input[(start - self.base) as usize..(self.pos - self.base) as usize]).unwrap()
    }
    /// Current absolute position (the offset of the next unread byte).
    fn loc(&self) -> u32 {
        self.pos
    }
    #[allow(clippy::cast_possible_truncation)]
    fn is_empty(&self) -> bool {
        self.pos - self.base >= self.input.len() as u32
    }
}
// move cursor to the end of the curent char
fn finish_char(cursor: &mut Cursor, start: u32) -> (char, u32) {
    while cursor.peek().is_some_and(|b| b & 0b1100_0000 == 0b1000_0000) {
        cursor.next();
    }
    let s = cursor.slice_from(start);
    let c = s.chars().next().expect(
        "finish_char: slice_from(start) was empty -- caller must consume the first byte \
         of the character before calling finish_char",
    );
    (c, cursor.loc())
}

fn parse_hex(cursor: &mut Cursor, open: u32) -> Result<u32, InternalLexerError> {
    if cursor.next() != Some(b'{') {
        return Err(lexer_error! {
            span = Span::new(open, open + 2),
            message = "expected '{{' after '\\x'",
            plabel = "at this escape sequence"
        })
    }

    let mut value: u32 = 0;
    let mut digit_count: u32 = 0;
    let digits_start = cursor.loc();
    // Set once we've detected a problem (too many digits, or value out of
    // range) but want to keep consuming hex digits so the eventual error
    // span covers the *entire* run of digits the user typed, not just the
    // prefix up to the first offending digit.
    let mut pending_error: Option<InternalLexerError> = None;

    loop {
        match cursor.peek() {
            Some(b'}') => {
                let close = cursor.loc();
                cursor.next();

                if digit_count == 0 {
                    // No digits between `{` and `}`.
                    return Err(lexer_error! {
                        span = Span::new(open, close),
                        message = "empty unicode escape expected non-empty hex between '{{' and '}}'",
                        plabel = "expected non-empty hex here",
                    });
                }

                if let Some(mut pending_error) = pending_error {
                    pending_error.span.hi = close; // extend to '}'
                    return Err(pending_error);
                }

                return Ok(value);
            }
            Some(b @ (b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')) => {
                cursor.next();
                digit_count += 1;

                if digit_count > 6 {
                    // keep scanning (without overflowing `value`, until `}` for better span.
                    pending_error.get_or_insert(lexer_error! {
                        span = Span::new(digits_start, cursor.pos),
                        message = "too many hex digits in unicode escape",
                        plabel = "too many hex digits here",
                        note = "at most 6 hex digits are allowed between '{' and '}' (codepoints only go up to 10FFFF), e.g. '\\x{10FFFF}'"
                    });
                    continue;
                }

                let digit = match b {
                    b'0'..=b'9' => b - b'0',
                    b'a'..=b'f' => b - b'a' + 10,
                    b'A'..=b'F' => b - b'A' + 10,
                    _ => unreachable!(),
                };
                value = (value << 4) | u32::from(digit);

                if value > 0x0010_FFFF {
                    // keep scanning (without overflowing `value`, until `}` for better span.
                    pending_error.get_or_insert(lexer_error! {
                        span = Span::new(digits_start, cursor.pos),
                        message = "unicode escape out of range",
                        plabel = "this isn't a valid codepoint",
                        note = "the maximum valid codepoint is U+10FFFF ('\\x{10FFFF}')"
                    });
                }
            }
            Some(_) => {
                // Not a hex digit and not '}'
                let start = cursor.loc();
                cursor.next();
                let (ch, end) = finish_char(cursor, start);
                return Err(lexer_error! {
                    span = Span::new(start, end),
                    message = format!("invalid hex digit '{}' in unicode escape: '", ch),
                    plabel = "inside this unicode escape",
                    note = "only the digits 0-9 and letters a-f/A-F are allowed inside '\\x{...}",
                });
            }
            None => {
                // Ran off the end of the literal before finding a closing '}'.
                let end = cursor.loc();
                return Err(lexer_error! {
                    span = Span::new(open, end),
                    message = "unterminated unicode escape",
                    plabel = "unicode escape unclosed here",
                    note = "expected a closing '}' before the end of the literal"
                });
            }
        }
    }
}

fn decode_escape(cursor: &mut Cursor) -> Result<char, InternalLexerError> {
    // `esc_start` is the position of the backslash itself; callers call
    // `cursor.next()` to consume the `\` *before* calling `decode_escape`,
    // so we reconstruct that position here for span purposes.
    let esc_start = cursor.loc() - 1;

    let b_pos = cursor.loc();
    let b = cursor.next().ok_or_else(|| {
        lexer_error! {
            span = Span::new(esc_start, esc_start),
            message = "dangling backslash",
            plabel = "closing quote is escaped here",
            note = "expected an escape character after '\\'",
        }
    })?;

    let esc = match b {
        b'n' => '\n',
        b't' => '\t',
        b'r' => '\r',
        b'\\' => '\\',
        b'\'' => '\'',
        b'"' => '"',
        b'0' => '\0',

        b'x' => {
            let esc_span = Span::new(esc_start, cursor.loc());
            let cp = parse_hex(cursor, esc_start)?;
            let end = cursor.loc();

            // Reject UTF-16 surrogate halves (U+D800–U+DFFF). These are not valid Unicode scalar values and cannot be 
            // represented a `char`.
            if (0xD800..=0xDFFF).contains(&cp) {
                return Err(lexer_error! {
                    span = Span::new(esc_start, end),
                    message = "invalid unicode escape",
                    plabel = "this escape produces a surrogate half",
                    note = format!("U+{cp:04X} is a UTF-16 surrogate half and is not a valid \
                                    Unicode scalar value; surrogate halves (U+D800–U+DFFF) \
                                    cannot be used in escape sequences")
                });
            }

            // char::from_u32 accepts all non-surrogate codepoints <= U+10FFFF so this unwrap cannot fail.
            let _ = esc_span; // span was only needed for the surrogate error above
            char::from_u32(cp).expect(
                "decode_escape: parse_hex returned a non-surrogate codepoint that char::from_u32 rejected \
                -- this is a bug",
            )
        }

        _ if b.is_ascii() => {
            let l = cursor.loc();
            return Err(lexer_error! {
                span = Span::new(esc_start, l),
                message = format!("unknown escape: '\\{}' is not a recognized escape sequence", b as char),
                plabel = "this isn't a recognized escape sequence",
                note = VALID_ESCAPES,
            });
        }

        _ => {
            // The byte after `\` is the first byte of a multi-byte UTF-8
            // character. Walk the rest of that character so both the reported character and the error span are correct
            let (ch, end) = finish_char(cursor, b_pos);
            return Err(lexer_error! {
                span = Span::new(esc_start, end),
                message = format!("unknown escape: '\\{ch}' is not a recognized escape sequence"),
                plabel = "this isn't a recognized escape sequence",
                note = VALID_ESCAPES,
            });
        }
    };

    Ok(esc)
}

pub fn parse_char<'a, 's>(lex: &'a mut LogosLexer<'s>) -> Result<u32, InternalLexerError> {
    let raw = lex.slice();
    let tok_span = current_span(lex);

    if raw == "''" {
        return Err(lexer_error!{
            span = tok_span,
            message = "empty character literal",
            plabel = "empty here",
            note = "a char literal must contain exactly one character",
        });
    }

    // Inner contents start 1 byte after the opening quote of the token.
    let inner_base = tok_span.lo + 1;
    let inner = &raw[1..raw.len() - 1];
    let mut cursor = Cursor::new(inner, inner_base);

    let value = match cursor.peek() {
        Some(b'\\') => {
            cursor.next();
            decode_escape(&mut cursor)?
        }
        Some(_) => {
            let start = cursor.loc();
            // Advance past one full UTF-8 character
            cursor.next();
            let (c, _end) = finish_char(&mut cursor, start);
            c
        }
        None => {
            return Err(lexer_error! {
                span = tok_span,
                message = "invalid char literal",
                plabel = "this is not a valid char literal",
            });
        }
    };

    if !cursor.is_empty() {
        // Multiple characters (or a character plus trailing junk) inside a single-quoted literal: 
        // span the *entire* literal, including both quotes
        return Err(lexer_error! {
            span = tok_span,
            message = "too many characters in char literal",
            plabel = "this literal contains more than one character",
            note = " a char literal must contain exactly one character; did you mean to use a string literal (\"...\") instead?",
        });
    }

    Ok(value as u32)
}

pub fn parse_str(lex: &mut LogosLexer<'_>) -> Result<String, InternalLexerError> {
    let raw = lex.slice();
    let tok_span = current_span(lex);

    let inner_base = tok_span.lo + 1;
    let inner = &raw[1..raw.len() - 1];

    let mut cursor = Cursor::new(inner, inner_base);
    let mut out = String::with_capacity(inner.len());

    while let Some(b) = cursor.next() {
        match b {
            b'\\' => {
                let esc = decode_escape(&mut cursor)?;
                out.push(esc);
            }

            // ASCII: push directly, no need to look at UTF-8 continuation bytes.
            b if b.is_ascii() => out.push(b as char),

            // Non-ASCII: walk back one byte (already consumed by `next()`) and re-decode the full UTF-8 sequence as a 
            // `&str`
            _ => {
                let start = cursor.loc() - 1;
                let (c, _end) = finish_char(&mut cursor, start);
                out.push(c);
            }
        }
    }

    Ok(out)
}
