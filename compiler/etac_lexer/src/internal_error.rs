use etac_span::Span;

macro_rules! lexer_error {
    (span = $span:expr, message = $message:expr $(, $key:ident = $val:expr)* $(,)?) => {{
        #[allow(unused_mut)]
        let mut err = $crate::InternalLexerError {
            span: $span,
            message: ::std::string::ToString::to_string(&$message),
            plabel: None,
            note: None,
        };
        $(
            $crate::lexer_error!(@set err, $key, $val);
        )*
        err
    }};

    (@set $err:ident, plabel, $val:expr) => {
        $err.plabel = ::std::option::Option::Some(::std::string::ToString::to_string(&$val));
    };
    (@set $err:ident, note, $val:expr) => {
        $err.note = ::std::option::Option::Some(::std::string::ToString::to_string(&$val));
    };
    (@set $err:ident, $unknown:ident, $val:expr) => {
        compile_error!(concat!("unknown lexer_err field: ", stringify!($unknown)));
    };
}

pub(crate) use lexer_error;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct InternalLexerError {
    pub span: Span,
    pub message: String,
    pub plabel: Option<String>,
    pub note: Option<String>,
}

impl Default for InternalLexerError {
    fn default() -> Self {
        panic!("Lexer should never use the default error")
    }
}
