use etac_errors::DiagCtxt;
use etac_lexer::Lexer;
use expect_test::{Expect, expect};
use std::fmt::Write as _;

fn lex(src: &str) -> String {
    let output = String::new();

    let mut out = String::new();
    for item in Lexer::new(0, src, &dcx) {
        match item {
            Ok((lo, tok, hi)) => {
                let _ = writeln!(out, "{lo}..{hi}  {tok:?}");
            }
            Err(diag) => {
                let loc = diag.loc;
                let message = diag.message.clone();
                // Defuse the diagnostic's drop bomb; we render it ourselves, never emit.
                diag.cancel();
                match loc {
                    Some(s) => {
                        let _ = writeln!(out, "{}..{}  error: {message}", s.lo, s.hi);
                    }
                    None => {
                        let _ = writeln!(out, "error: {message}");
                    }
                }
                break;
            }
        }
    }
    out
}

fn check(src: &str, expected: Expect) {
    expected.assert_eq(&lex(src));
}

// ── Cases: etac_tests/pa1 ───────────────────────────────────────────

#[test]
fn add() {
    check(include_str!("cases/pa1/add.eta"), expect![[r#"
        0..1  OperatorAdd
        1..2  OperatorAdd
        2..3  OperatorAdd
    "#]]);
}

#[test]
fn arrayinit() {
    check(include_str!("cases/pa1/arrayinit.eta"), expect![[r#"
        0..1  Identifier("a")
        1..2  OfType
        3..6  KeywordInt
        6..7  LBracket
        7..8  RBracket
        9..10  Assign
        11..12  BlockOpen
        12..14  Integer(72)
        14..15  Comma
        15..18  Integer(101)
        18..19  Comma
        19..22  Integer(108)
        22..23  Comma
        23..26  Integer(108)
        26..27  Comma
        27..30  Integer(111)
        30..31  BlockClose
        32..33  Identifier("a")
        33..34  OfType
        35..38  KeywordInt
        38..39  LBracket
        39..40  RBracket
        41..42  Assign
        43..50  StrLiteral("Hello")
    "#]]);
}

#[test]
fn arrayinit2() {
    check(include_str!("cases/pa1/arrayinit2.eta"), expect![[r#"
        0..1  Identifier("n")
        1..2  OfType
        3..6  KeywordInt
        7..8  Assign
        9..12  Identifier("gcd")
        12..13  LParen
        13..15  Integer(10)
        15..16  Comma
        17..18  Integer(2)
        18..19  RParen
        20..21  Identifier("a")
        21..22  OfType
        23..26  KeywordInt
        26..27  LBracket
        27..28  Identifier("n")
        28..29  RBracket
        30..35  KeywordWhile
        36..37  LParen
        37..38  Identifier("n")
        39..40  RelOpGr
        41..42  Integer(0)
        42..43  RParen
        44..45  BlockOpen
        48..49  Identifier("n")
        50..51  Assign
        52..53  Identifier("n")
        54..55  Minus
        56..57  Integer(1)
        60..61  Identifier("a")
        61..62  LBracket
        62..63  Identifier("n")
        63..64  RBracket
        65..66  Assign
        67..68  Identifier("n")
        69..70  BlockClose
    "#]]);
}

#[test]
fn badescape() {
    check(include_str!("cases/pa1/badescape.eta"), expect![[r#"
        32..38  error: unicode escape out of range
    "#]]);
}

#[test]
fn beauty() {
    check(include_str!("cases/pa1/beauty.eta"), expect![[r#"
        0..2  RelOpEq
        2..4  RelOpEq
        4..6  RelOpEq
        6..8  RelOpEq
        8..10  RelOpEq
        10..12  RelOpEq
        12..14  RelOpEq
        14..16  RelOpEq
        16..18  RelOpEq
        18..20  RelOpEq
        20..22  RelOpEq
        22..24  RelOpEq
        24..26  RelOpEq
        26..28  RelOpEq
        28..30  RelOpEq
        30..32  RelOpEq
        32..34  RelOpEq
        34..36  RelOpEq
        36..38  RelOpEq
        38..40  RelOpEq
        40..42  RelOpEq
        42..44  RelOpEq
        44..46  RelOpEq
        46..48  RelOpEq
        48..50  RelOpEq
        50..52  RelOpEq
        52..54  RelOpEq
        54..56  RelOpEq
        56..58  RelOpEq
        58..60  RelOpEq
        60..62  RelOpEq
        62..64  RelOpEq
        64..66  RelOpEq
        66..68  RelOpEq
        68..70  RelOpEq
        70..72  RelOpEq
        72..74  RelOpEq
        74..76  RelOpEq
        76..78  RelOpEq
        78..79  Assign
        80..81  Assign
        82..86  Identifier("This")
        87..89  Identifier("is")
        90..91  Identifier("a")
        92..101  Identifier("beautiful")
        102..110  Identifier("document")
        111..118  Identifier("heading")
        118..119  Comma
        120..123  Identifier("not")
        124..125  Identifier("a")
        126..128  Identifier("xi")
        129..136  Identifier("program")
        136..137  Comma
        138..141  Identifier("but")
        142..144  Identifier("it")
        145..150  Identifier("still")
        151..156  Identifier("lexes")
        156..157  OperatorNot
        158..159  Assign
        160..162  RelOpEq
        162..164  RelOpEq
        164..166  RelOpEq
        166..168  RelOpEq
        168..170  RelOpEq
        170..172  RelOpEq
        172..174  RelOpEq
        174..176  RelOpEq
        176..178  RelOpEq
        178..180  RelOpEq
        180..182  RelOpEq
        182..184  RelOpEq
        184..186  RelOpEq
        186..188  RelOpEq
        188..190  RelOpEq
        190..192  RelOpEq
        192..194  RelOpEq
        194..196  RelOpEq
        196..198  RelOpEq
        198..200  RelOpEq
        200..202  RelOpEq
        202..204  RelOpEq
        204..206  RelOpEq
        206..208  RelOpEq
        208..210  RelOpEq
        210..212  RelOpEq
        212..214  RelOpEq
        214..216  RelOpEq
        216..218  RelOpEq
        218..220  RelOpEq
        220..222  RelOpEq
        222..224  RelOpEq
        224..226  RelOpEq
        226..228  RelOpEq
        228..230  RelOpEq
        230..232  RelOpEq
        232..234  RelOpEq
        234..236  RelOpEq
        236..238  RelOpEq
        238..239  Assign
    "#]]);
}

#[test]
fn consecutive_operators() {
    check(include_str!("cases/pa1/consecutive_operators.eta"), expect![[r#"
        0..1  Identifier("a")
        1..2  OperatorAdd
        2..3  Minus
        3..4  Identifier("b")
        5..6  OperatorNot
        6..7  Minus
        7..8  Identifier("x")
        9..10  Minus
        10..11  Minus
        11..12  Identifier("x")
    "#]]);
}

#[test]
fn empty_hex_escape() {
    check(include_str!("cases/pa1/empty_hex_escape.eta"), expect![[r#"
        1..4  error: empty unicode escape expected non-empty hex between '{{' and '}}'
    "#]]);
}

#[test]
fn error_escape_q() {
    check(include_str!("cases/pa1/error_escape_q.eta"), expect![[r#"
        1..3  error: unknown escape: '\q' is not a recognized escape sequence
    "#]]);
}

#[test]
fn error_escape_space() {
    check(include_str!("cases/pa1/error_escape_space.eta"), expect![[r#"
        1..3  error: unknown escape: '\ ' is not a recognized escape sequence
    "#]]);
}

#[test]
fn error_escape_x() {
    check(include_str!("cases/pa1/error_escape_x.eta"), expect![[r#"
        1..3  error: expected '{{' after '\x'
    "#]]);
}

#[test]
fn escape_double_quote() {
    check(include_str!("cases/pa1/escape_double_quote.eta"), expect![[r#"
        0..4  CharLiteral(34)
        5..18  StrLiteral("say \"hi\"!")
    "#]]);
}

#[test]
fn escape_hex_newline() {
    check(include_str!("cases/pa1/escape_hex_newline.eta"), expect![[r#"
        0..8  CharLiteral(10)
        9..17  StrLiteral("\n")
    "#]]);
}

#[test]
fn escape_null_cr() {
    check(include_str!("cases/pa1/escape_null_cr.eta"), expect![[r#"
        0..4  CharLiteral(0)
        5..9  CharLiteral(13)
        10..16  StrLiteral("\0\r")
    "#]]);
}

#[test]
fn escapes() {
    check(include_str!("cases/pa1/escapes.eta"), expect![[r#"
        0..20  StrLiteral("Hello, World!")
        59..67  CharLiteral(100)
        68..88  StrLiteral("Hello, Worlµ!")
        120..128  CharLiteral(181)
        129..133  StrLiteral("\t")
        134..138  CharLiteral(9)
        139..145  StrLiteral("éç")
        147..149  error: unknown escape: '\q' is not a recognized escape sequence
    "#]]);
}

#[test]
fn ex1() {
    check(include_str!("cases/pa1/ex1.eta"), expect![[r#"
        0..3  KeywordUse
        4..6  Identifier("io")
        8..12  Identifier("main")
        12..13  LParen
        13..17  Identifier("args")
        17..18  OfType
        19..22  KeywordInt
        22..23  LBracket
        23..24  RBracket
        24..25  LBracket
        25..26  RBracket
        26..27  RParen
        28..29  BlockOpen
        32..37  Identifier("print")
        37..38  LParen
        38..60  StrLiteral("Hello, World!\n")
        60..61  RParen
        64..68  Identifier("c3po")
        68..69  OfType
        70..73  KeywordInt
        74..75  Assign
        76..79  CharLiteral(120)
        80..81  OperatorAdd
        82..84  Integer(47)
        84..85  SemiColon
        88..92  Identifier("r2d2")
        92..93  OfType
        94..97  KeywordInt
        98..99  Assign
        100..104  Identifier("c3po")
        120..121  BlockClose
    "#]]);
}

#[test]
fn ex2() {
    check(include_str!("cases/pa1/ex2.eta"), expect![[r#"
        0..1  Identifier("x")
        1..2  OfType
        2..6  KeywordBool
        7..8  Assign
        9..10  Integer(4)
        10..13  Identifier("all")
        14..15  Identifier("x")
        16..17  Assign
        18..20  error: empty character literal
    "#]]);
}

#[test]
fn gcd() {
    check(include_str!("cases/pa1/gcd.eta"), expect![[r#"
        54..57  Identifier("gcd")
        57..58  LParen
        58..59  Identifier("a")
        59..60  OfType
        60..63  KeywordInt
        63..64  Comma
        65..66  Identifier("b")
        66..67  OfType
        67..70  KeywordInt
        70..71  RParen
        71..72  OfType
        72..75  KeywordInt
        76..77  BlockOpen
        80..85  KeywordWhile
        86..87  LParen
        87..88  Identifier("a")
        89..91  RelOpNeq
        92..93  Integer(0)
        93..94  RParen
        95..96  BlockOpen
        101..103  KeywordIf
        104..105  LParen
        105..106  Identifier("a")
        106..107  RelOpLt
        107..108  Identifier("b")
        108..109  RParen
        110..111  Identifier("b")
        112..113  Assign
        114..115  Identifier("b")
        116..117  Minus
        118..119  Identifier("a")
        124..128  KeywordElse
        129..130  Identifier("a")
        131..132  Assign
        133..134  Identifier("a")
        135..136  Minus
        137..138  Identifier("b")
        141..142  BlockClose
        145..151  KeywordReturn
        151..152  LParen
        152..153  Identifier("b")
        153..154  RParen
        155..156  BlockClose
    "#]]);
}

#[test]
fn high_mul_operator() {
    check(include_str!("cases/pa1/high_mul_operator.eta"), expect![[r#"
        0..1  Identifier("a")
        2..5  OperatorHighMul
        6..7  Identifier("b")
    "#]]);
}

#[test]
fn identifier_with_primes() {
    check(include_str!("cases/pa1/identifier_with_primes.eta"), expect![[r#"
        0..2  Identifier("x'")
        3..6  Identifier("a'b")
        7..11  Identifier("x'_1")
        12..17  Identifier("q'r's")
    "#]]);
}

#[test]
fn insertionsort() {
    check(include_str!("cases/pa1/insertionsort.eta"), expect![[r#"
        0..4  Identifier("sort")
        4..5  LParen
        5..6  Identifier("a")
        6..7  OfType
        8..11  KeywordInt
        11..12  LBracket
        12..13  RBracket
        13..14  RParen
        15..16  BlockOpen
        19..20  Identifier("i")
        20..21  OfType
        21..24  KeywordInt
        25..26  Assign
        27..28  Integer(0)
        31..32  Identifier("n")
        32..33  OfType
        33..36  KeywordInt
        37..38  Assign
        39..45  KeywordLength
        45..46  LParen
        46..47  Identifier("a")
        47..48  RParen
        51..56  KeywordWhile
        57..58  LParen
        58..59  Identifier("i")
        60..61  RelOpLt
        62..63  Identifier("n")
        63..64  RParen
        65..66  BlockOpen
        73..74  Identifier("j")
        74..75  OfType
        75..78  KeywordInt
        79..80  Assign
        81..82  Identifier("i")
        89..94  KeywordWhile
        95..96  LParen
        96..97  Identifier("j")
        98..99  RelOpGr
        100..101  Integer(0)
        101..102  RParen
        103..104  BlockOpen
        113..115  KeywordIf
        116..117  LParen
        117..118  Identifier("a")
        118..119  LBracket
        119..120  Identifier("j")
        120..122  Integer(-1)
        122..123  RBracket
        124..125  RelOpGr
        126..127  Identifier("a")
        127..128  LBracket
        128..129  Identifier("j")
        129..130  RBracket
        130..131  RParen
        132..133  BlockOpen
        146..150  Identifier("swap")
        150..151  OfType
        151..154  KeywordInt
        155..156  Assign
        157..158  Identifier("a")
        158..159  LBracket
        159..160  Identifier("j")
        160..161  RBracket
        174..175  Identifier("a")
        175..176  LBracket
        176..177  Identifier("j")
        177..178  RBracket
        179..180  Assign
        181..182  Identifier("a")
        182..183  LBracket
        183..184  Identifier("j")
        184..186  Integer(-1)
        186..187  RBracket
        200..201  Identifier("a")
        201..202  LBracket
        202..203  Identifier("j")
        203..205  Integer(-1)
        205..206  RBracket
        207..208  Assign
        209..213  Identifier("swap")
        222..223  BlockClose
        232..233  Identifier("j")
        234..235  Assign
        236..237  Identifier("j")
        237..239  Integer(-1)
        246..247  BlockClose
        254..255  Identifier("i")
        256..257  Assign
        258..259  Identifier("i")
        259..260  OperatorAdd
        260..261  Integer(1)
        264..265  BlockClose
        266..267  BlockClose
    "#]]);
}

#[test]
fn interface() {
    check(include_str!("cases/pa1/interface.eti"), expect![[r#"
        19..22  Identifier("add")
        22..23  LParen
        23..24  Identifier("a")
        24..25  OfType
        26..29  KeywordInt
        29..30  Comma
        31..32  Identifier("b")
        32..33  OfType
        34..37  KeywordInt
        37..38  RParen
        38..39  OfType
        40..43  KeywordInt
        44..50  Identifier("matrix")
        50..51  LParen
        51..52  Identifier("m")
        52..53  OfType
        54..57  KeywordInt
        57..58  LBracket
        58..59  RBracket
        59..60  LBracket
        60..61  RBracket
        61..62  RParen
        62..63  OfType
        64..67  KeywordInt
        67..68  LBracket
        68..69  RBracket
        69..70  LBracket
        70..71  RBracket
        72..80  Identifier("noreturn")
        80..81  LParen
        81..82  Identifier("a")
        82..83  OfType
        84..87  KeywordInt
        87..88  RParen
        89..95  Identifier("noargs")
        95..96  LParen
        96..97  RParen
        97..98  OfType
        99..102  KeywordInt
    "#]]);
}

#[test]
fn intoverflow() {
    check(include_str!("cases/pa1/intoverflow.eta"), expect![[r#"
        0..23  error: illegal integer literal: number too large to fit in target type
    "#]]);
}

#[test]
fn keyword_prefix_identifiers() {
    check(include_str!("cases/pa1/keyword_prefix_identifiers.eta"), expect![[r#"
        0..7  Identifier("integer")
        7..8  OfType
        9..12  KeywordInt
        13..14  Assign
        15..16  Integer(0)
        17..24  Identifier("boolean")
        24..25  OfType
        26..30  KeywordBool
        31..32  Assign
        33..37  BoolLiteral(true)
        38..42  Identifier("uses")
        42..43  OfType
        44..47  KeywordInt
        48..49  Assign
        50..51  Integer(1)
        52..55  Identifier("iff")
        55..56  OfType
        57..60  KeywordInt
        61..62  Assign
        63..64  Integer(2)
        65..74  Identifier("whileLoop")
        74..75  OfType
        76..79  KeywordInt
        80..81  Assign
        82..83  Integer(3)
        84..93  Identifier("returning")
        93..94  OfType
        95..98  KeywordInt
        99..100  Assign
        101..102  Integer(4)
        103..110  Identifier("lengths")
        110..111  OfType
        112..115  KeywordInt
        116..117  Assign
        118..119  Integer(5)
    "#]]);
}

#[test]
fn large_int() {
    check(include_str!("cases/pa1/large_int.eta"), expect![[r#"
        0..31  error: illegal integer literal: number too large to fit in target type
    "#]]);
}

#[test]
fn leading_zeros() {
    check(include_str!("cases/pa1/leading_zeros.eta"), expect![[r#"
        0..1  Integer(0)
        1..2  Integer(0)
        3..4  Integer(0)
        4..5  Integer(1)
    "#]]);
}

#[test]
fn max_int_boundary() {
    check(include_str!("cases/pa1/max_int_boundary.eta"), expect![[r#"
        0..19  Integer(9223372036854775807)
        20..39  error: illegal integer literal: number too large to fit in target type
    "#]]);
}

#[test]
fn max_valid_codepoint() {
    check(include_str!("cases/pa1/max_valid_codepoint.eta"), expect![[r#"
        0..12  CharLiteral(1114111)
    "#]]);
}

#[test]
fn mdarrays() {
    check(include_str!("cases/pa1/mdarrays.eta"), expect![[r#"
        0..1  Identifier("a")
        1..2  OfType
        3..6  KeywordInt
        6..7  LBracket
        7..8  RBracket
        8..9  LBracket
        9..10  RBracket
        11..12  Identifier("b")
        12..13  OfType
        14..17  KeywordInt
        17..18  LBracket
        18..19  Integer(3)
        19..20  RBracket
        20..21  LBracket
        21..22  Integer(4)
        22..23  RBracket
        24..25  Identifier("a")
        26..27  Assign
        28..29  Identifier("b")
        30..31  Identifier("c")
        31..32  OfType
        33..36  KeywordInt
        36..37  LBracket
        37..38  Integer(3)
        38..39  RBracket
        39..40  LBracket
        40..41  RBracket
        42..43  Identifier("c")
        43..44  LBracket
        44..45  Integer(0)
        45..46  RBracket
        47..48  Assign
        49..50  Identifier("b")
        50..51  LBracket
        51..52  Integer(0)
        52..53  RBracket
        53..54  SemiColon
        55..56  Identifier("c")
        56..57  LBracket
        57..58  Integer(1)
        58..59  RBracket
        60..61  Assign
        62..63  Identifier("b")
        63..64  LBracket
        64..65  Integer(1)
        65..66  RBracket
        66..67  SemiColon
        68..69  Identifier("c")
        69..70  LBracket
        70..71  Integer(2)
        71..72  RBracket
        73..74  Assign
        75..76  Identifier("b")
        76..77  LBracket
        77..78  Integer(2)
        78..79  RBracket
        80..81  Identifier("d")
        81..82  OfType
        83..86  KeywordInt
        86..87  LBracket
        87..88  RBracket
        88..89  LBracket
        89..90  RBracket
        91..92  Assign
        93..94  BlockOpen
        94..95  BlockOpen
        95..96  Integer(1)
        96..97  Comma
        98..99  Integer(0)
        99..100  BlockClose
        100..101  Comma
        102..103  BlockOpen
        103..104  Integer(0)
        104..105  Comma
        106..107  Integer(1)
        107..108  BlockClose
        108..109  BlockClose
    "#]]);
}

#[test]
fn modulo_operator() {
    check(include_str!("cases/pa1/modulo_operator.eta"), expect![[r#"
        0..1  Identifier("a")
        2..3  OperatorMod
        4..5  Identifier("b")
    "#]]);
}

#[test]
fn multiline_string() {
    check(include_str!("cases/pa1/multiline_string.eta"), expect![[r#"
        0..13  StrLiteral("Hello\nWorld")
    "#]]);
}

#[test]
fn ratadd() {
    check(include_str!("cases/pa1/ratadd.eta"), expect![[r#"
        104..110  Identifier("ratadd")
        110..111  LParen
        111..113  Identifier("p1")
        113..114  OfType
        114..117  KeywordInt
        117..118  Comma
        119..121  Identifier("q1")
        121..122  OfType
        122..125  KeywordInt
        125..126  Comma
        127..129  Identifier("p2")
        129..130  OfType
        130..133  KeywordInt
        133..134  Comma
        135..137  Identifier("q2")
        137..138  OfType
        138..141  KeywordInt
        141..142  RParen
        143..144  OfType
        145..146  LParen
        146..149  KeywordInt
        149..150  Comma
        151..154  KeywordInt
        154..155  RParen
        156..157  BlockOpen
        162..163  Identifier("g")
        163..164  OfType
        164..167  KeywordInt
        168..169  Assign
        170..173  Identifier("gcd")
        173..174  LParen
        174..176  Identifier("q1")
        176..177  Comma
        177..179  Identifier("q2")
        179..180  RParen
        185..187  Identifier("p3")
        187..188  OfType
        188..191  KeywordInt
        192..193  Assign
        194..196  Identifier("p1")
        196..197  OperatorMul
        197..198  LParen
        198..200  Identifier("q2")
        200..201  OperatorDiv
        201..202  Identifier("g")
        202..203  RParen
        204..205  OperatorAdd
        206..208  Identifier("p2")
        208..209  OperatorMul
        209..210  LParen
        210..212  Identifier("q1")
        212..213  OperatorDiv
        213..214  Identifier("g")
        214..215  RParen
        220..226  KeywordReturn
        227..228  LParen
        228..230  Identifier("p3")
        230..231  Comma
        232..234  Identifier("q1")
        234..235  OperatorDiv
        235..236  Identifier("g")
        236..237  OperatorMul
        237..239  Identifier("q2")
        239..240  RParen
        241..242  BlockClose
    "#]]);
}

#[test]
fn ratadduse() {
    check(include_str!("cases/pa1/ratadduse.eta"), expect![[r#"
        0..1  LParen
        1..2  Identifier("p")
        2..3  OfType
        3..6  KeywordInt
        6..7  Comma
        8..9  Identifier("q")
        9..10  OfType
        10..13  KeywordInt
        13..14  RParen
        15..16  Assign
        17..23  Identifier("ratadd")
        23..24  LParen
        24..25  Integer(2)
        25..26  Comma
        27..28  Integer(5)
        28..29  Comma
        30..31  Integer(1)
        31..32  Comma
        33..34  Integer(3)
        34..35  RParen
        36..37  LParen
        37..38  Discard
        38..39  Comma
        40..42  Identifier("q'")
        42..43  OfType
        43..46  KeywordInt
        46..47  RParen
        48..49  Assign
        50..56  Identifier("ratadd")
        56..57  LParen
        57..58  Integer(1)
        58..59  Comma
        60..61  Integer(2)
        61..62  Comma
        63..64  Integer(1)
        64..65  Comma
        66..67  Integer(3)
        67..68  RParen
    "#]]);
}

#[test]
fn spec1() {
    check(include_str!("cases/pa1/spec1.eta"), expect![[r#"
        0..1  Identifier("x")
        1..2  OfType
        2..5  KeywordInt
        6..7  Assign
        8..9  Integer(2)
        9..10  SemiColon
        11..12  Identifier("z")
        12..13  OfType
        13..16  KeywordInt
        16..17  SemiColon
        18..19  LParen
        19..20  Identifier("b")
        20..21  OfType
        22..26  KeywordBool
        26..27  Comma
        28..29  Identifier("i")
        29..30  OfType
        30..33  KeywordInt
        33..34  RParen
        35..36  Assign
        37..38  Identifier("f")
        38..39  LParen
        39..40  Identifier("x")
        40..41  RParen
        41..42  SemiColon
        43..44  Identifier("s")
        44..45  OfType
        46..49  KeywordInt
        49..50  LBracket
        50..51  RBracket
        52..53  Assign
        54..61  StrLiteral("Hello")
        61..62  SemiColon
    "#]]);
}

#[test]
fn spec2() {
    check(include_str!("cases/pa1/spec2.eta"), expect![[r#"
        2..3  Identifier("x")
        4..5  Assign
        6..7  Identifier("x")
        8..9  OperatorAdd
        10..11  Integer(1)
        14..15  Identifier("s")
        16..17  Assign
        18..19  BlockOpen
        19..20  Integer(1)
        20..21  Comma
        22..23  Integer(2)
        23..24  Comma
        25..26  Integer(3)
        26..27  BlockClose
        30..31  Identifier("b")
        32..33  Assign
        34..35  OperatorNot
        35..36  Identifier("b")
    "#]]);
}

#[test]
fn spec3() {
    check(include_str!("cases/pa1/spec3.eta"), expect![[r#"
        0..1  Identifier("s")
        1..2  OfType
        3..6  KeywordInt
        6..7  LBracket
        7..8  RBracket
        9..10  Assign
        11..18  StrLiteral("Hello")
        19..20  OperatorAdd
        21..22  BlockOpen
        22..24  Integer(13)
        24..25  Comma
        26..28  Integer(10)
        28..29  BlockClose
    "#]]);
}

#[test]
fn string_tab() {
    check(include_str!("cases/pa1/string_tab.eta"), expect![[r#"
        0..4  StrLiteral("\t")
    "#]]);
}

#[test]
fn supplementary_char() {
    check(include_str!("cases/pa1/supplementary_char.eta"), expect![[r#"
        0..4  Identifier("main")
        4..5  LParen
        5..9  Identifier("args")
        9..10  OfType
        11..14  KeywordInt
        14..15  LBracket
        15..16  RBracket
        16..17  LBracket
        17..18  RBracket
        18..19  RParen
        20..21  BlockOpen
        26..27  Identifier("a")
        27..28  OfType
        28..31  KeywordInt
        32..33  Assign
        34..40  CharLiteral(128512)
        40..41  SemiColon
        46..47  Identifier("b")
        47..48  OfType
        48..51  KeywordInt
        52..53  Assign
        54..60  CharLiteral(127344)
        60..61  SemiColon
        66..67  Identifier("c")
        67..68  OfType
        68..71  KeywordInt
        71..72  LBracket
        72..73  RBracket
        74..75  Assign
        76..94  StrLiteral("Hello 😀 World")
        94..95  SemiColon
        96..97  BlockClose
    "#]]);
}

#[test]
fn supplementary_outofbounds() {
    check(include_str!("cases/pa1/supplementary_outofbounds.eta"), expect![[r#"
        0..4  Identifier("main")
        4..5  LParen
        5..9  Identifier("args")
        9..10  OfType
        11..14  KeywordInt
        14..15  LBracket
        15..16  RBracket
        16..17  LBracket
        17..18  RBracket
        18..19  RParen
        20..21  BlockOpen
        26..28  Identifier("ok")
        28..29  OfType
        30..33  KeywordInt
        33..34  LBracket
        34..35  RBracket
        36..37  Assign
        42..48  error: unicode escape out of range
    "#]]);
}

#[test]
fn surrogate_char() {
    check(include_str!("cases/pa1/surrogate_char.eta"), expect![[r#"
        1..9  error: invalid unicode escape
    "#]]);
}

#[test]
fn surrogate_string() {
    check(include_str!("cases/pa1/surrogate_string.eta"), expect![[r#"
        1..9  error: invalid unicode escape
    "#]]);
}

#[test]
fn unclosedescape() {
    check(include_str!("cases/pa1/unclosedescape.eta"), expect![[r#"
        12..17  error: unterminated unicode escape
    "#]]);
}

#[test]
fn unicode_bmp_char() {
    check(include_str!("cases/pa1/unicode_bmp_char.eta"), expect![[r#"
        0..4  CharLiteral(233)
    "#]]);
}

