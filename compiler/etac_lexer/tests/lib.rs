#![allow(unused_assignments)]

use expect_test::expect;
use indoc::indoc;

#[macro_use]
pub mod lexer;

#[test]
fn add() {
    lextest! {
        "+++",
        expect![[r#"
            src0:1:1..1:2 +
            src0:1:2..1:3 +
            src0:1:3..1:4 +
        "#]]
    }
}
#[test]
fn arrayinit() {
    lextest! {
        indoc![r#"
            a: int[] = {72,101,108,108,111}
            a: int[] = "Hello"
        "#],
        expect![[r#"
            src0:1:1..1:2 a
            src0:1:2..1:3 :
            src0:1:4..1:7 int
            src0:1:7..1:8 [
            src0:1:8..1:9 ]
            src0:1:10..1:11 =
            src0:1:12..1:13 {
            src0:1:13..1:15 72
            src0:1:15..1:16 ,
            src0:1:16..1:19 101
            src0:1:19..1:20 ,
            src0:1:20..1:23 108
            src0:1:23..1:24 ,
            src0:1:24..1:27 108
            src0:1:27..1:28 ,
            src0:1:28..1:31 111
            src0:1:31..1:32 }
            src0:2:1..2:2 a
            src0:2:2..2:3 :
            src0:2:4..2:7 int
            src0:2:7..2:8 [
            src0:2:8..2:9 ]
            src0:2:10..2:11 =
            src0:2:12..2:19 "Hello"
        "#]]
    }
}
#[test]
fn arrayinit2() {
    lextest! {
        indoc![r#"
            n: int = gcd(10, 2)
            a: int[n]
            while (n > 0) {
              n = n - 1
              a[n] = n
            }
        "#],
        expect![[r#"
            src0:1:1..1:2 n
            src0:1:2..1:3 :
            src0:1:4..1:7 int
            src0:1:8..1:9 =
            src0:1:10..1:13 gcd
            src0:1:13..1:14 (
            src0:1:14..1:16 10
            src0:1:16..1:17 ,
            src0:1:18..1:19 2
            src0:1:19..1:20 )
            src0:2:1..2:2 a
            src0:2:2..2:3 :
            src0:2:4..2:7 int
            src0:2:7..2:8 [
            src0:2:8..2:9 n
            src0:2:9..2:10 ]
            src0:3:1..3:6 while
            src0:3:7..3:8 (
            src0:3:8..3:9 n
            src0:3:10..3:11 >
            src0:3:12..3:13 0
            src0:3:13..3:14 )
            src0:3:15..3:16 {
            src0:4:3..4:4 n
            src0:4:5..4:6 =
            src0:4:7..4:8 n
            src0:4:9..4:10 -
            src0:4:11..4:12 1
            src0:5:3..5:4 a
            src0:5:4..5:5 [
            src0:5:5..5:6 n
            src0:5:6..5:7 ]
            src0:5:8..5:9 =
            src0:5:10..5:11 n
            src0:6:1..6:2 }
        "#]]
    }
}
#[test]
fn badescape() {
    lextest! {
        indoc![r#"
            This is not a Unicode char: \x{ffffff}
        "#], 
        expect![[r#"
            src0:1:1..1:5 This
            src0:1:6..1:8 is
            src0:1:9..1:12 not
            src0:1:13..1:14 a
            src0:1:15..1:22 Unicode
            src0:1:23..1:27 char
            src0:1:27..1:28 :
            src0:1:30..1:31 x
            src0:1:31..1:32 {
            src0:1:32..1:38 ffffff
            src0:1:38..1:39 }
            Error: unknown token
               ╭─[ src0:1:29 ]
               │
             1 │ This is not a Unicode char: \x{ffffff}
               │                             ┬  
               │                             ╰── this token
            ───╯
        "#]]
    }
}
#[test]
fn beauty() {
    lextest! {
        indoc![r#"
            ===============================================================================
            = This is a beautiful document heading, not a xi program, but it still lexes! =
            ===============================================================================
        "#],
        expect![[r#"
            src0:1:1..1:3 ==
            src0:1:3..1:5 ==
            src0:1:5..1:7 ==
            src0:1:7..1:9 ==
            src0:1:9..1:11 ==
            src0:1:11..1:13 ==
            src0:1:13..1:15 ==
            src0:1:15..1:17 ==
            src0:1:17..1:19 ==
            src0:1:19..1:21 ==
            src0:1:21..1:23 ==
            src0:1:23..1:25 ==
            src0:1:25..1:27 ==
            src0:1:27..1:29 ==
            src0:1:29..1:31 ==
            src0:1:31..1:33 ==
            src0:1:33..1:35 ==
            src0:1:35..1:37 ==
            src0:1:37..1:39 ==
            src0:1:39..1:41 ==
            src0:1:41..1:43 ==
            src0:1:43..1:45 ==
            src0:1:45..1:47 ==
            src0:1:47..1:49 ==
            src0:1:49..1:51 ==
            src0:1:51..1:53 ==
            src0:1:53..1:55 ==
            src0:1:55..1:57 ==
            src0:1:57..1:59 ==
            src0:1:59..1:61 ==
            src0:1:61..1:63 ==
            src0:1:63..1:65 ==
            src0:1:65..1:67 ==
            src0:1:67..1:69 ==
            src0:1:69..1:71 ==
            src0:1:71..1:73 ==
            src0:1:73..1:75 ==
            src0:1:75..1:77 ==
            src0:1:77..1:79 ==
            src0:1:79..1:80 =
            src0:2:1..2:2 =
            src0:2:3..2:7 This
            src0:2:8..2:10 is
            src0:2:11..2:12 a
            src0:2:13..2:22 beautiful
            src0:2:23..2:31 document
            src0:2:32..2:39 heading
            src0:2:39..2:40 ,
            src0:2:41..2:44 not
            src0:2:45..2:46 a
            src0:2:47..2:49 xi
            src0:2:50..2:57 program
            src0:2:57..2:58 ,
            src0:2:59..2:62 but
            src0:2:63..2:65 it
            src0:2:66..2:71 still
            src0:2:72..2:77 lexes
            src0:2:77..2:78 !
            src0:2:79..2:80 =
            src0:3:1..3:3 ==
            src0:3:3..3:5 ==
            src0:3:5..3:7 ==
            src0:3:7..3:9 ==
            src0:3:9..3:11 ==
            src0:3:11..3:13 ==
            src0:3:13..3:15 ==
            src0:3:15..3:17 ==
            src0:3:17..3:19 ==
            src0:3:19..3:21 ==
            src0:3:21..3:23 ==
            src0:3:23..3:25 ==
            src0:3:25..3:27 ==
            src0:3:27..3:29 ==
            src0:3:29..3:31 ==
            src0:3:31..3:33 ==
            src0:3:33..3:35 ==
            src0:3:35..3:37 ==
            src0:3:37..3:39 ==
            src0:3:39..3:41 ==
            src0:3:41..3:43 ==
            src0:3:43..3:45 ==
            src0:3:45..3:47 ==
            src0:3:47..3:49 ==
            src0:3:49..3:51 ==
            src0:3:51..3:53 ==
            src0:3:53..3:55 ==
            src0:3:55..3:57 ==
            src0:3:57..3:59 ==
            src0:3:59..3:61 ==
            src0:3:61..3:63 ==
            src0:3:63..3:65 ==
            src0:3:65..3:67 ==
            src0:3:67..3:69 ==
            src0:3:69..3:71 ==
            src0:3:71..3:73 ==
            src0:3:73..3:75 ==
            src0:3:75..3:77 ==
            src0:3:77..3:79 ==
            src0:3:79..3:80 =
        "#]]
    }
}
#[test]
fn consecutive_operators() {
    lextest! {
        indoc![r#"
            a+-b
            !-x
            --x
        "#],
        expect![[r#"
            src0:1:1..1:2 a
            src0:1:2..1:3 +
            src0:1:3..1:4 -
            src0:1:4..1:5 b
            src0:2:1..2:2 !
            src0:2:2..2:3 -
            src0:2:3..2:4 x
            src0:3:1..3:2 -
            src0:3:2..3:3 -
            src0:3:3..3:4 x
        "#]]
    }
}
#[test]
fn empty_hex_escape() {
    lextest! {
        indoc![r#"
            "\x{}"
        "#],
        expect![[r#"
            Error: empty unicode escape expected non-empty hex between '{' and '}'
               ╭─[ src0:1:2 ]
               │
             1 │ "\x{}"
               │  ─┬─  
               │   ╰─── expected non-empty hex here
            ───╯
        "#]]
    }
}
#[test]
fn error_escape_q() {
    lextest! {
        indoc![r#"
            "\q"
        "#],
        expect![[r#"
            Error: unknown escape: '\q' is not a recognized escape sequence
               ╭─[ src0:1:2 ]
               │
             1 │ "\q"
               │  ─┬  
               │   ╰── this isn't a recognized escape sequence
               │ 
               │ Note: valid escapes: '\n', '\t', '\r', '\\', '\'', '\"', '\0', '\x{..}'
            ───╯
        "#]]
    }
}
#[test]
fn error_escape_space() {
    lextest! {
        indoc![r#"
            "\ \ \ "
        "#],
        expect![[r#"
            Error: unknown escape: '\ ' is not a recognized escape sequence
               ╭─[ src0:1:2 ]
               │
             1 │ "\ \ \ "
               │  ─┬  
               │   ╰── this isn't a recognized escape sequence
               │ 
               │ Note: valid escapes: '\n', '\t', '\r', '\\', '\'', '\"', '\0', '\x{..}'
            ───╯
        "#]]
    }
}
#[test]
fn error_escape_x() {
    lextest! {
        indoc![r#"
            "\x"
        "#],
        expect![[r#"
            Error: expected '{{' after '\x'
               ╭─[ src0:1:2 ]
               │
             1 │ "\x"
               │  ─┬  
               │   ╰── at this escape sequence
            ───╯
        "#]]
    }
}
#[test]
fn escape_double_quote() {
    lextest! {
        indoc![r#"
            '\"'
            "say \"hi\"!"
        "#],
        expect![[r#"
            src0:1:1..1:5 '\"'
            src0:2:1..2:14 "say \"hi\"!"
        "#]]
    }
}
#[test]
fn escape_hex_newline() {
    lextest! {
        indoc![r#"
            '\x{0a}'
            "\x{0a}"
        "#],
        expect![[r#"
            src0:1:1..1:9 '\n'
            src0:2:1..2:9 "\n"
        "#]]
    }
}
#[test]
fn escape_null_cr() {
    lextest! {
        indoc![r#"
            '\0'
            '\r'
            "\0\r"
        "#],
        expect![[r#"
            src0:1:1..1:5 '\x{0}'
            src0:2:1..2:5 '\r'
            src0:3:1..3:7 "\x{0}\r"
        "#]]
    }
}
#[test]
fn escapes() {
    lextest! {
        indoc![r#"
            "Hello, Worl\x{64}!" 
            '\x{64}'
            "Hello, Worl\x{b5}!" 
            '\x{b5}'
            "\t"
            '\t'
            "éç"
            "\q" 
        "#],
        expect![[r#"
            src0:1:1..1:21 "Hello, World!"
            src0:2:1..2:9 'd'
            src0:3:1..3:21 "Hello, Worl\x{b5}!"
            src0:4:1..4:9 '\x{b5}'
            src0:5:1..5:5 "\t"
            src0:6:1..6:5 '\t'
            src0:7:1..7:7 "\x{e9}\x{e7}"
            Error: unknown escape: '\q' is not a recognized escape sequence
               ╭─[ src0:8:2 ]
               │
             8 │ "\q"
               │  ─┬  
               │   ╰── this isn't a recognized escape sequence
               │ 
               │ Note: valid escapes: '\n', '\t', '\r', '\\', '\'', '\"', '\0', '\x{..}'
            ───╯
        "#]]
    }
}
#[test]
fn ex1() {
    lextest! {
        indoc![r#"
            use io

            main(args: int[][]) {
              print("Hello, Worl\x{64}!\n")
              c3po: int = 'x' + 47;
              r2d2: int = c3po 
            }
        "#],
        expect![[r#"
            src0:1:1..1:4 use
            src0:1:5..1:7 io
            src0:3:1..3:5 main
            src0:3:5..3:6 (
            src0:3:6..3:10 args
            src0:3:10..3:11 :
            src0:3:12..3:15 int
            src0:3:15..3:16 [
            src0:3:16..3:17 ]
            src0:3:17..3:18 [
            src0:3:18..3:19 ]
            src0:3:19..3:20 )
            src0:3:21..3:22 {
            src0:4:3..4:8 print
            src0:4:8..4:9 (
            src0:4:9..4:31 "Hello, World!\n"
            src0:4:31..4:32 )
            src0:5:3..5:7 c3po
            src0:5:7..5:8 :
            src0:5:9..5:12 int
            src0:5:13..5:14 =
            src0:5:15..5:18 'x'
            src0:5:19..5:20 +
            src0:5:21..5:23 47
            src0:5:23..5:24 ;
            src0:6:3..6:7 r2d2
            src0:6:7..6:8 :
            src0:6:9..6:12 int
            src0:6:13..6:14 =
            src0:6:15..6:19 c3po
            src0:7:1..7:2 }
        "#]]
    }
}
#[test]
fn ex2() {
    lextest! {
        indoc![r#"
            x:bool = 4all
            x = ''
            this = does not matter
        "#],
        expect![[r#"
            src0:1:1..1:2 x
            src0:1:2..1:3 :
            src0:1:3..1:7 bool
            src0:1:8..1:9 =
            src0:1:10..1:11 4
            src0:1:11..1:14 all
            src0:2:1..2:2 x
            src0:2:3..2:4 =
            src0:3:1..3:5 this
            src0:3:6..3:7 =
            src0:3:8..3:12 does
            src0:3:13..3:16 not
            src0:3:17..3:23 matter
            Error: empty character literal
               ╭─[ src0:2:5 ]
               │
             2 │ x = ''
               │     ─┬  
               │      ╰── empty here
               │ 
               │ Note: a char literal must contain exactly one character
            ───╯
        "#]]
    }
}
#[test]
fn gcd() {
    lextest! {
        indoc![r#"
            gcd(a:int, b:int):int {
              while (a != 0) {
                if (a<b) b = b - a
                else a = a - b
              }
              return(b)
            }
        "#],
        expect![[r#"
            src0:1:1..1:4 gcd
            src0:1:4..1:5 (
            src0:1:5..1:6 a
            src0:1:6..1:7 :
            src0:1:7..1:10 int
            src0:1:10..1:11 ,
            src0:1:12..1:13 b
            src0:1:13..1:14 :
            src0:1:14..1:17 int
            src0:1:17..1:18 )
            src0:1:18..1:19 :
            src0:1:19..1:22 int
            src0:1:23..1:24 {
            src0:2:3..2:8 while
            src0:2:9..2:10 (
            src0:2:10..2:11 a
            src0:2:12..2:14 !=
            src0:2:15..2:16 0
            src0:2:16..2:17 )
            src0:2:18..2:19 {
            src0:3:5..3:7 if
            src0:3:8..3:9 (
            src0:3:9..3:10 a
            src0:3:10..3:11 <
            src0:3:11..3:12 b
            src0:3:12..3:13 )
            src0:3:14..3:15 b
            src0:3:16..3:17 =
            src0:3:18..3:19 b
            src0:3:20..3:21 -
            src0:3:22..3:23 a
            src0:4:5..4:9 else
            src0:4:10..4:11 a
            src0:4:12..4:13 =
            src0:4:14..4:15 a
            src0:4:16..4:17 -
            src0:4:18..4:19 b
            src0:5:3..5:4 }
            src0:6:3..6:9 return
            src0:6:9..6:10 (
            src0:6:10..6:11 b
            src0:6:11..6:12 )
            src0:7:1..7:2 }
        "#]]
    }
}
#[test]
fn high_mul_operator() {
    lextest! {
        indoc![r#"
            a *>> b
        "#],
        expect![[r#"
            src0:1:1..1:2 a
            src0:1:3..1:6 *>>
            src0:1:7..1:8 b
        "#]]
    }
}
#[test]
fn identifier_with_primes() {
    lextest! {
        indoc![r#"
            x'
            a'b
            x'_1
            q'r's
        "#],
        expect![[r#"
            src0:1:1..1:3 x'
            src0:2:1..2:4 a'b
            src0:3:1..3:5 x'_1
            src0:4:1..4:6 q'r's
        "#]]
    }
}
#[test]
fn insertionsort() {
    lextest! {
        indoc![r#"
            sort(a: int[]) {
              i:int = 0
              n:int = length(a)
              while (i < n) {
                  j:int = i
                  while (j > 0) {
                    if (a[j-1] > a[j]) {
                        swap:int = a[j]
                        a[j] = a[j-1]
                        a[j-1] = swap
                    }
                    j = j-1
                  }
                  i = i+1
              }
            }
        "#],
        expect![[r#"
            src0:1:1..1:5 sort
            src0:1:5..1:6 (
            src0:1:6..1:7 a
            src0:1:7..1:8 :
            src0:1:9..1:12 int
            src0:1:12..1:13 [
            src0:1:13..1:14 ]
            src0:1:14..1:15 )
            src0:1:16..1:17 {
            src0:2:3..2:4 i
            src0:2:4..2:5 :
            src0:2:5..2:8 int
            src0:2:9..2:10 =
            src0:2:11..2:12 0
            src0:3:3..3:4 n
            src0:3:4..3:5 :
            src0:3:5..3:8 int
            src0:3:9..3:10 =
            src0:3:11..3:17 length
            src0:3:17..3:18 (
            src0:3:18..3:19 a
            src0:3:19..3:20 )
            src0:4:3..4:8 while
            src0:4:9..4:10 (
            src0:4:10..4:11 i
            src0:4:12..4:13 <
            src0:4:14..4:15 n
            src0:4:15..4:16 )
            src0:4:17..4:18 {
            src0:5:7..5:8 j
            src0:5:8..5:9 :
            src0:5:9..5:12 int
            src0:5:13..5:14 =
            src0:5:15..5:16 i
            src0:6:7..6:12 while
            src0:6:13..6:14 (
            src0:6:14..6:15 j
            src0:6:16..6:17 >
            src0:6:18..6:19 0
            src0:6:19..6:20 )
            src0:6:21..6:22 {
            src0:7:9..7:11 if
            src0:7:12..7:13 (
            src0:7:13..7:14 a
            src0:7:14..7:15 [
            src0:7:15..7:16 j
            src0:7:16..7:17 -
            src0:7:17..7:18 1
            src0:7:18..7:19 ]
            src0:7:20..7:21 >
            src0:7:22..7:23 a
            src0:7:23..7:24 [
            src0:7:24..7:25 j
            src0:7:25..7:26 ]
            src0:7:26..7:27 )
            src0:7:28..7:29 {
            src0:8:13..8:17 swap
            src0:8:17..8:18 :
            src0:8:18..8:21 int
            src0:8:22..8:23 =
            src0:8:24..8:25 a
            src0:8:25..8:26 [
            src0:8:26..8:27 j
            src0:8:27..8:28 ]
            src0:9:13..9:14 a
            src0:9:14..9:15 [
            src0:9:15..9:16 j
            src0:9:16..9:17 ]
            src0:9:18..9:19 =
            src0:9:20..9:21 a
            src0:9:21..9:22 [
            src0:9:22..9:23 j
            src0:9:23..9:24 -
            src0:9:24..9:25 1
            src0:9:25..9:26 ]
            src0:10:13..10:14 a
            src0:10:14..10:15 [
            src0:10:15..10:16 j
            src0:10:16..10:17 -
            src0:10:17..10:18 1
            src0:10:18..10:19 ]
            src0:10:20..10:21 =
            src0:10:22..10:26 swap
            src0:11:9..11:10 }
            src0:12:9..12:10 j
            src0:12:11..12:12 =
            src0:12:13..12:14 j
            src0:12:14..12:15 -
            src0:12:15..12:16 1
            src0:13:7..13:8 }
            src0:14:7..14:8 i
            src0:14:9..14:10 =
            src0:14:11..14:12 i
            src0:14:12..14:13 +
            src0:14:13..14:14 1
            src0:15:3..15:4 }
            src0:16:1..16:2 }
        "#]]
    }
}
#[test]
fn interface() {
    lextest! {
        indoc![r#"
            
            add(a: int, b: int): int
            matrix(m: int[][]): int[][]
            noreturn(a: int)
            noargs(): int
        "#],
        expect![[r#"
            src0:2:1..2:4 add
            src0:2:4..2:5 (
            src0:2:5..2:6 a
            src0:2:6..2:7 :
            src0:2:8..2:11 int
            src0:2:11..2:12 ,
            src0:2:13..2:14 b
            src0:2:14..2:15 :
            src0:2:16..2:19 int
            src0:2:19..2:20 )
            src0:2:20..2:21 :
            src0:2:22..2:25 int
            src0:3:1..3:7 matrix
            src0:3:7..3:8 (
            src0:3:8..3:9 m
            src0:3:9..3:10 :
            src0:3:11..3:14 int
            src0:3:14..3:15 [
            src0:3:15..3:16 ]
            src0:3:16..3:17 [
            src0:3:17..3:18 ]
            src0:3:18..3:19 )
            src0:3:19..3:20 :
            src0:3:21..3:24 int
            src0:3:24..3:25 [
            src0:3:25..3:26 ]
            src0:3:26..3:27 [
            src0:3:27..3:28 ]
            src0:4:1..4:9 noreturn
            src0:4:9..4:10 (
            src0:4:10..4:11 a
            src0:4:11..4:12 :
            src0:4:13..4:16 int
            src0:4:16..4:17 )
            src0:5:1..5:7 noargs
            src0:5:7..5:8 (
            src0:5:8..5:9 )
            src0:5:9..5:10 :
            src0:5:11..5:14 int
        "#]]
    }
}
#[test]
fn int_overflow() {
    lextest! {
        indoc![r#"
            12393498732984798273498
        "#],
        expect![[r#"
            Error: illegal integer literal: this integer literal is too large to be stored in any Eta type
               ╭─[ src0:1:1 ]
               │
             1 │ 12393498732984798273498
               │ ───────────┬───────────  
               │            ╰───────────── this integer is too large
               │ 
               │ Note: eta only supports ints in the range [-2^63, 2^63)
            ───╯
        "#]]
    }
}
#[test]
fn keyword_prefix_identifiers() {
    lextest! {
        indoc![r#"
            integer: int = 0
            boolean: bool = true
            uses: int = 1
            iff: int = 2
            whileLoop: int = 3
            returning: int = 4
            lengths: int = 5
        "#],
        expect![[r#"
            src0:1:1..1:8 integer
            src0:1:8..1:9 :
            src0:1:10..1:13 int
            src0:1:14..1:15 =
            src0:1:16..1:17 0
            src0:2:1..2:8 boolean
            src0:2:8..2:9 :
            src0:2:10..2:14 bool
            src0:2:15..2:16 =
            src0:2:17..2:21 true
            src0:3:1..3:5 uses
            src0:3:5..3:6 :
            src0:3:7..3:10 int
            src0:3:11..3:12 =
            src0:3:13..3:14 1
            src0:4:1..4:4 iff
            src0:4:4..4:5 :
            src0:4:6..4:9 int
            src0:4:10..4:11 =
            src0:4:12..4:13 2
            src0:5:1..5:10 whileLoop
            src0:5:10..5:11 :
            src0:5:12..5:15 int
            src0:5:16..5:17 =
            src0:5:18..5:19 3
            src0:6:1..6:10 returning
            src0:6:10..6:11 :
            src0:6:12..6:15 int
            src0:6:16..6:17 =
            src0:6:18..6:19 4
            src0:7:1..7:8 lengths
            src0:7:8..7:9 :
            src0:7:10..7:13 int
            src0:7:14..7:15 =
            src0:7:16..7:17 5
        "#]]
    }
}
#[test]
fn large_int() {
    lextest! {
        indoc![r#"
            1000000000000000000000000000000
        "#],
        expect![[r#"
            Error: illegal integer literal: this integer literal is too large to be stored in any Eta type
               ╭─[ src0:1:1 ]
               │
             1 │ 1000000000000000000000000000000
               │ ───────────────┬───────────────  
               │                ╰───────────────── this integer is too large
               │ 
               │ Note: eta only supports ints in the range [-2^63, 2^63)
            ───╯
        "#]]
    }
}
#[test]
fn leading_zeros() {
    lextest! {
        indoc![r#"
            00
            01
        "#],
        expect![[r#"
            src0:1:1..1:2 0
            src0:1:2..1:3 0
            src0:2:1..2:2 0
            src0:2:2..2:3 1
        "#]]
    }
}
#[test]
fn max_int_boundary() {
    lextest! {
        indoc![r#"
            9223372036854775807
            9223372036854775808
        "#],
        expect![[r#"
            src0:1:1..1:20 9223372036854775807
            src0:2:1..2:20 9223372036854775808
        "#]]
    }
}
#[test]
fn max_valid_codepoint() {
    lextest! {
        indoc![r#"
            '\x{10FFFF}'
        "#],
        expect![[r#"
            src0:1:1..1:13 '\x{10ffff}'
        "#]]
    }
}
#[test]
fn mdarrays() {
    lextest! {
        indoc![r#"
            a: int[][]
            b: int[3][4]
            a = b
            c: int[3][]
            c[0] = b[0]; c[1] = b[1]; c[2] = b[2]
            d: int[][] = {{1, 0}, {0, 1}}
        "#],
        expect![[r#"
            src0:1:1..1:2 a
            src0:1:2..1:3 :
            src0:1:4..1:7 int
            src0:1:7..1:8 [
            src0:1:8..1:9 ]
            src0:1:9..1:10 [
            src0:1:10..1:11 ]
            src0:2:1..2:2 b
            src0:2:2..2:3 :
            src0:2:4..2:7 int
            src0:2:7..2:8 [
            src0:2:8..2:9 3
            src0:2:9..2:10 ]
            src0:2:10..2:11 [
            src0:2:11..2:12 4
            src0:2:12..2:13 ]
            src0:3:1..3:2 a
            src0:3:3..3:4 =
            src0:3:5..3:6 b
            src0:4:1..4:2 c
            src0:4:2..4:3 :
            src0:4:4..4:7 int
            src0:4:7..4:8 [
            src0:4:8..4:9 3
            src0:4:9..4:10 ]
            src0:4:10..4:11 [
            src0:4:11..4:12 ]
            src0:5:1..5:2 c
            src0:5:2..5:3 [
            src0:5:3..5:4 0
            src0:5:4..5:5 ]
            src0:5:6..5:7 =
            src0:5:8..5:9 b
            src0:5:9..5:10 [
            src0:5:10..5:11 0
            src0:5:11..5:12 ]
            src0:5:12..5:13 ;
            src0:5:14..5:15 c
            src0:5:15..5:16 [
            src0:5:16..5:17 1
            src0:5:17..5:18 ]
            src0:5:19..5:20 =
            src0:5:21..5:22 b
            src0:5:22..5:23 [
            src0:5:23..5:24 1
            src0:5:24..5:25 ]
            src0:5:25..5:26 ;
            src0:5:27..5:28 c
            src0:5:28..5:29 [
            src0:5:29..5:30 2
            src0:5:30..5:31 ]
            src0:5:32..5:33 =
            src0:5:34..5:35 b
            src0:5:35..5:36 [
            src0:5:36..5:37 2
            src0:5:37..5:38 ]
            src0:6:1..6:2 d
            src0:6:2..6:3 :
            src0:6:4..6:7 int
            src0:6:7..6:8 [
            src0:6:8..6:9 ]
            src0:6:9..6:10 [
            src0:6:10..6:11 ]
            src0:6:12..6:13 =
            src0:6:14..6:15 {
            src0:6:15..6:16 {
            src0:6:16..6:17 1
            src0:6:17..6:18 ,
            src0:6:19..6:20 0
            src0:6:20..6:21 }
            src0:6:21..6:22 ,
            src0:6:23..6:24 {
            src0:6:24..6:25 0
            src0:6:25..6:26 ,
            src0:6:27..6:28 1
            src0:6:28..6:29 }
            src0:6:29..6:30 }
        "#]]
    }
}
#[test]
fn modulo_operator() {
    lextest! {
        indoc![r#"
            a % b
        "#],
        expect![[r#"
            src0:1:1..1:2 a
            src0:1:3..1:4 %
            src0:1:5..1:6 b
        "#]]
    }
}
#[test]
fn multiline_string() {
    lextest! {
        indoc![r#"
            "Hello
            World"
        "#],
        expect![[r#"
            src0:1:1..2:7 "Hello\nWorld"
        "#]]
    }
}
#[test]
fn ratadd() {
    lextest! {
        indoc![r#"
            ratadd(p1:int, q1:int, p2:int, q2:int) : (int, int) {
                g:int = gcd(q1,q2)
                p3:int = p1*(q2/g) + p2*(q1/g)
                return (p3, q1/g*q2)
            }
        "#],
        expect![[r#"
            src0:1:1..1:7 ratadd
            src0:1:7..1:8 (
            src0:1:8..1:10 p1
            src0:1:10..1:11 :
            src0:1:11..1:14 int
            src0:1:14..1:15 ,
            src0:1:16..1:18 q1
            src0:1:18..1:19 :
            src0:1:19..1:22 int
            src0:1:22..1:23 ,
            src0:1:24..1:26 p2
            src0:1:26..1:27 :
            src0:1:27..1:30 int
            src0:1:30..1:31 ,
            src0:1:32..1:34 q2
            src0:1:34..1:35 :
            src0:1:35..1:38 int
            src0:1:38..1:39 )
            src0:1:40..1:41 :
            src0:1:42..1:43 (
            src0:1:43..1:46 int
            src0:1:46..1:47 ,
            src0:1:48..1:51 int
            src0:1:51..1:52 )
            src0:1:53..1:54 {
            src0:2:5..2:6 g
            src0:2:6..2:7 :
            src0:2:7..2:10 int
            src0:2:11..2:12 =
            src0:2:13..2:16 gcd
            src0:2:16..2:17 (
            src0:2:17..2:19 q1
            src0:2:19..2:20 ,
            src0:2:20..2:22 q2
            src0:2:22..2:23 )
            src0:3:5..3:7 p3
            src0:3:7..3:8 :
            src0:3:8..3:11 int
            src0:3:12..3:13 =
            src0:3:14..3:16 p1
            src0:3:16..3:17 *
            src0:3:17..3:18 (
            src0:3:18..3:20 q2
            src0:3:20..3:21 /
            src0:3:21..3:22 g
            src0:3:22..3:23 )
            src0:3:24..3:25 +
            src0:3:26..3:28 p2
            src0:3:28..3:29 *
            src0:3:29..3:30 (
            src0:3:30..3:32 q1
            src0:3:32..3:33 /
            src0:3:33..3:34 g
            src0:3:34..3:35 )
            src0:4:5..4:11 return
            src0:4:12..4:13 (
            src0:4:13..4:15 p3
            src0:4:15..4:16 ,
            src0:4:17..4:19 q1
            src0:4:19..4:20 /
            src0:4:20..4:21 g
            src0:4:21..4:22 *
            src0:4:22..4:24 q2
            src0:4:24..4:25 )
            src0:5:1..5:2 }
        "#]]
    }
}
#[test]
fn lex_pa1_ratadduse() {
    lextest! {
        indoc![r#"
            (p:int, q:int) = ratadd(2, 5, 1, 3)
            (_, q':int) = ratadd(1, 2, 1, 3)
        "#],
        expect![[r#"
            src0:1:1..1:2 (
            src0:1:2..1:3 p
            src0:1:3..1:4 :
            src0:1:4..1:7 int
            src0:1:7..1:8 ,
            src0:1:9..1:10 q
            src0:1:10..1:11 :
            src0:1:11..1:14 int
            src0:1:14..1:15 )
            src0:1:16..1:17 =
            src0:1:18..1:24 ratadd
            src0:1:24..1:25 (
            src0:1:25..1:26 2
            src0:1:26..1:27 ,
            src0:1:28..1:29 5
            src0:1:29..1:30 ,
            src0:1:31..1:32 1
            src0:1:32..1:33 ,
            src0:1:34..1:35 3
            src0:1:35..1:36 )
            src0:2:1..2:2 (
            src0:2:2..2:3 _
            src0:2:3..2:4 ,
            src0:2:5..2:7 q'
            src0:2:7..2:8 :
            src0:2:8..2:11 int
            src0:2:11..2:12 )
            src0:2:13..2:14 =
            src0:2:15..2:21 ratadd
            src0:2:21..2:22 (
            src0:2:22..2:23 1
            src0:2:23..2:24 ,
            src0:2:25..2:26 2
            src0:2:26..2:27 ,
            src0:2:28..2:29 1
            src0:2:29..2:30 ,
            src0:2:31..2:32 3
            src0:2:32..2:33 )
        "#]]
    }
}
#[test]
fn spec1() {
    lextest! {
        indoc![r#"
            x:int = 2;
            z:int;
            (b: bool, i:int) = f(x}
            s: int[] = "Hello";
        "#],
        expect![[r#"
            src0:1:1..1:2 x
            src0:1:2..1:3 :
            src0:1:3..1:6 int
            src0:1:7..1:8 =
            src0:1:9..1:10 2
            src0:1:10..1:11 ;
            src0:2:1..2:2 z
            src0:2:2..2:3 :
            src0:2:3..2:6 int
            src0:2:6..2:7 ;
            src0:3:1..3:2 (
            src0:3:2..3:3 b
            src0:3:3..3:4 :
            src0:3:5..3:9 bool
            src0:3:9..3:10 ,
            src0:3:11..3:12 i
            src0:3:12..3:13 :
            src0:3:13..3:16 int
            src0:3:16..3:17 )
            src0:3:18..3:19 =
            src0:3:20..3:21 f
            src0:3:21..3:22 (
            src0:3:22..3:23 x
            src0:3:23..3:24 }
            src0:4:1..4:2 s
            src0:4:2..4:3 :
            src0:4:4..4:7 int
            src0:4:7..4:8 [
            src0:4:8..4:9 ]
            src0:4:10..4:11 =
            src0:4:12..4:19 "Hello"
            src0:4:19..4:20 ;
        "#]]
    }
}
#[test]
fn spec2() {
    lextest! {
        indoc![r#"
          x = x + 1
          s = {1, 2, 3}
          b = !b
        "#],
        expect![[r#"
            src0:1:1..1:2 x
            src0:1:3..1:4 =
            src0:1:5..1:6 x
            src0:1:7..1:8 +
            src0:1:9..1:10 1
            src0:2:1..2:2 s
            src0:2:3..2:4 =
            src0:2:5..2:6 {
            src0:2:6..2:7 1
            src0:2:7..2:8 ,
            src0:2:9..2:10 2
            src0:2:10..2:11 ,
            src0:2:12..2:13 3
            src0:2:13..2:14 }
            src0:3:1..3:2 b
            src0:3:3..3:4 =
            src0:3:5..3:6 !
            src0:3:6..3:7 b
        "#]]
    }
}
#[test]
fn spec3() {
    lextest! {
        indoc![r#"
            s: int[] = "Hello" + {13, 10}
        "#],
        expect![[r#"
            src0:1:1..1:2 s
            src0:1:2..1:3 :
            src0:1:4..1:7 int
            src0:1:7..1:8 [
            src0:1:8..1:9 ]
            src0:1:10..1:11 =
            src0:1:12..1:19 "Hello"
            src0:1:20..1:21 +
            src0:1:22..1:23 {
            src0:1:23..1:25 13
            src0:1:25..1:26 ,
            src0:1:27..1:29 10
            src0:1:29..1:30 }
        "#]]
    }
}
#[test]
fn string_tab() {
    lextest! {
        indoc![r#"
            "\t"
        "#],
        expect![[r#"
            src0:1:1..1:5 "\t"
        "#]]
    }
}
#[test]
fn supplementary_char() {
    lextest! {
        indoc![r#"
            main(args: int[][]) {
                a:int = '😀';
                b:int = '🅰';
                c:int[] = "Hello 😀 World";
            }
        "#],
        expect![[r#"
            src0:1:1..1:5 main
            src0:1:5..1:6 (
            src0:1:6..1:10 args
            src0:1:10..1:11 :
            src0:1:12..1:15 int
            src0:1:15..1:16 [
            src0:1:16..1:17 ]
            src0:1:17..1:18 [
            src0:1:18..1:19 ]
            src0:1:19..1:20 )
            src0:1:21..1:22 {
            src0:2:5..2:6 a
            src0:2:6..2:7 :
            src0:2:7..2:10 int
            src0:2:11..2:12 =
            src0:2:13..2:19 '\x{1f600}'
            src0:2:19..2:20 ;
            src0:3:5..3:6 b
            src0:3:6..3:7 :
            src0:3:7..3:10 int
            src0:3:11..3:12 =
            src0:3:13..3:19 '\x{1f170}'
            src0:3:19..3:20 ;
            src0:4:5..4:6 c
            src0:4:6..4:7 :
            src0:4:7..4:10 int
            src0:4:10..4:11 [
            src0:4:11..4:12 ]
            src0:4:13..4:14 =
            src0:4:15..4:33 "Hello \x{1f600} World"
            src0:4:33..4:34 ;
            src0:5:1..5:2 }
        "#]]
    }
}
#[test]
fn supplementary_outofbounds() {
    lextest! {
        indoc![r#"
            main(args: int[][]) {
                ok: int[] = "\x{110000}"; 
            }
        "#],
        expect![[r#"
            src0:1:1..1:5 main
            src0:1:5..1:6 (
            src0:1:6..1:10 args
            src0:1:10..1:11 :
            src0:1:12..1:15 int
            src0:1:15..1:16 [
            src0:1:16..1:17 ]
            src0:1:17..1:18 [
            src0:1:18..1:19 ]
            src0:1:19..1:20 )
            src0:1:21..1:22 {
            src0:2:5..2:7 ok
            src0:2:7..2:8 :
            src0:2:9..2:12 int
            src0:2:12..2:13 [
            src0:2:13..2:14 ]
            src0:2:15..2:16 =
            src0:2:29..2:30 ;
            src0:3:1..3:2 }
            Error: unicode escape out of range
               ╭─[ src0:2:21 ]
               │
             2 │     ok: int[] = "\x{110000}";
               │                     ───┬──  
               │                        ╰──── this isn't a valid codepoint
               │ 
               │ Note: the maximum valid codepoint is U+10FFFF ('\x{10FFFF}')
            ───╯
        "#]]
    }
}
#[test]
fn surrogate_char() {
    lextest! {
        indoc![r#"
            '\x{D800}'
        "#],
        expect![[r#"
            Error: invalid unicode escape
               ╭─[ src0:1:2 ]
               │
             1 │ '\x{D800}'
               │  ────┬───  
               │      ╰───── this escape produces a surrogate half
               │ 
               │ Note: U+D800 is a UTF-16 surrogate half and is not a valid Unicode scalar value; surrogate halves (U+D800–U+DFFF) cannot be used in escape sequences
            ───╯
        "#]]
    }
}
#[test]
fn surrogate_string() {
    lextest! {
        indoc![r#"
            "\x{D800}"
        "#],
        expect![[r#"
            Error: invalid unicode escape
               ╭─[ src0:1:2 ]
               │
             1 │ "\x{D800}"
               │  ────┬───  
               │      ╰───── this escape produces a surrogate half
               │ 
               │ Note: U+D800 is a UTF-16 surrogate half and is not a valid Unicode scalar value; surrogate halves (U+D800–U+DFFF) cannot be used in escape sequences
            ───╯
        "#]]
    }
}
#[test]
fn unclosedescape() {
    lextest! {
        indoc![r#"
            "Hello, Worl\x{64"
        "#],
        expect![[r#"
            Error: unterminated unicode escape
               ╭─[ src0:1:13 ]
               │
             1 │ "Hello, Worl\x{64"
               │             ──┬──  
               │               ╰──── unicode escape unclosed here
               │ 
               │ Note: expected a closing '}' before the end of the literal
            ───╯
        "#]]
    }
}
#[test]
fn unicode_bmp_char() {
    lextest! {
        indoc![r#"
            'é'
        "#],
        expect![[r#"
            src0:1:1..1:5 '\x{e9}'
        "#]]
    }
}
