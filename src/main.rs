//! calc — Lesson 1 scaffold: the input loop (Part A) and the lexer (Part B)
//!
//! HOW TO USE THIS FILE
//!   - Work top to bottom. TODOs are numbered in the order to do them:
//!     A1, A2, A3 (Part A), then B1 to B5 (Part B).
//!   - Each TODO names the tool you need. The Lesson 1 walkthrough explains it.
//!   - `todo!()` is a real Rust macro. It compiles, but panics with its
//!     message if that line ever runs. Replace each one with your code.
//!   - This file compiles as-is, so you can run `cargo check` after every
//!     few lines and fix errors while they're small.
//!
//! COMMANDS (run from the `calc` folder)
//!   cargo check   fast compiler feedback without building
//!   cargo run     run the calculator (Ctrl+D or `exit` to quit)
//!   cargo test    run the tests at the bottom of this file. They all fail
//!                 until Part B is written. Turning them green is the goal.
//!   cargo clippy  suggestions for more idiomatic Rust

// Silences "unused" warnings while the file is half-built.
// TODO (last step): delete this line once Part B is wired into main,
// then run `cargo check` and fix any warnings that appear.
#![allow(dead_code, unused_variables)]

// TODO A1: uncomment the line below.
//   `self`  imports the io module itself, so you can write io::stdin().
//   `Write` brings the Write trait into scope, which is what makes
//           .flush() exist. (Later, try deleting `Write` and read the error.)
//
use std::io::{self, Write};

// ============================================================================
// THE TOKEN TYPE (given: the tests depend on these exact names)
// Read every line and make sure you could explain it.
// ============================================================================

/// One meaningful unit of input.
/// The lexer turns "3 + 4" into [Num(3.0), Plus, Num(4.0)].
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Num(f64), // a number, carrying its value
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    LParen,   // (
    RParen,   // )
}

// ============================================================================
// PART A: THE INPUT LOOP
// ============================================================================

fn main() {
    println!("calc: type an expression, or `exit` to quit");

    // TODO A2: create the input buffer here, OUTSIDE the loop, so the same
    //          memory gets reused for every line.
    //   Tool: String::new()
    //   It has to be `let mut`, because read_line writes into it.

    let mut buf = String::new();

    // TODO A3: write the loop.    Tool: loop { ... }
    //   Inside it, in this order:
    //
    //   a) Print the prompt "> " WITHOUT a newline, then flush stdout.
    //        Tools: print!   and   io::stdout().flush()
    //        flush() returns a Result; `.expect("flush failed")` is fine here.
    //
    //   b) Clear the buffer.
    //        Tool: buf.clear()
    //
    //   c) Read a line, and `match` on the Result with three arms, in this order:
    //        Ok(0)  => end of input: print an empty line, then break
    //        Ok(_)  => {}   (got a line: do nothing, carry on below the match)
    //        Err(e) => print the error with eprintln!, then break
    //        Tool: io::stdin().read_line(&mut buf)
    //
    //   d) Get a trimmed view of the line:
    //        let input = buf.trim();
    //
    //   e) If input == "exit", break.
    //      If input.is_empty(), continue.
    //
    //   f) Echo the line back with println!. That finishes Part A:
    //      `cargo run` and try it.
    
    loop {
        print!("> ");          // Print and flush
        io::stdout().flush().expect("Failed to flush stdout");

        buf.clear();           // Clear buffer

        match io::stdin().read_line(&mut buf) {

            Ok(0) => {         // end of input
                println!();
                break;
            }

            Ok(_) => {}

            Err(e) => {        // Print error if one.
                eprintln!("Error reading line: {e}");
                break;
            }

        }

        let input = buf.trim();

        if input == "exit"{
            break;
        }
        if input.is_empty() {
            continue;
        }

        println!("Processed line: {input}");
    }

    // TODO B5 (after tokenize passes its tests): replace the echo in (f) with
    //   match tokenize(input) {
    //       Ok(tokens) => print them with {tokens:?}
    //       Err(msg)   => print "error: {msg}"
    //   }
}

// ============================================================================
// PART B: THE LEXER
// ============================================================================

/// Turn raw text into a list of tokens, or an error message.
///
/// The lexer does NOT check grammar: "3 + + 4" lexes fine here, because each
/// piece is a valid token. Catching that mistake is the parser's job (Lesson 2).
fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    // TODO B1: make an empty, growable list for the tokens.
    //   Tool: Vec::new()    (must be `let mut`, because you'll push into it)

    let mut tokens: Vec<Token> = Vec::new();

    // TODO B2: make a peekable iterator over the characters.
    //   Tool: input.chars().peekable()    (also `let mut`)

    let mut chars = input.chars().peekable();

    // TODO B3: the main loop.
    //   Tool: while let Some(&c) = chars.peek() { ... }
    //
    //   Inside it, `match c` with these arms:
    //
    //   - whitespace: use a guard pattern,  c if c.is_whitespace() =>
    //       consume it with chars.next(), and push nothing.
    //
    //   - '+'  '-'  '*'  '/'  '('  ')':  one arm per character
    //       consume it with chars.next(), then push the matching Token,
    //       e.g. tokens.push(Token::Plus)
    //
    //   - a digit or a dot:  '0'..='9' | '.' =>
    //       this is the number branch:
    //         1. make an empty String:   let mut text = String::new();
    //         2. inner loop:   while let Some(&d) = chars.peek() { ... }
    //              if d is a digit or a dot
    //                  (d.is_ascii_digit() || d == '.'),
    //              consume it and append it:   text.push(d)
    //              otherwise, break. Leave d UNCONSUMED: it belongs to the
    //              next token, and the outer loop will handle it.
    //         3. parse the text into a number:
    //              text.parse::<f64>()
    //              .map_err(|e| format!("bad number '{text}': {e}"))?
    //            The `?` returns the error from tokenize if parsing failed.
    //         4. push Token::Num(value)
    //
    //   - anything else:  _ =>
    //       return Err(format!("unexpected character '{c}'"));
    //
    //   WARNING: peek() never advances the iterator. Every branch that handles
    //   a character must call chars.next() (or let the inner loop do it), or
    //   the outer loop sees the same character forever.

    while let Some(&c) = chars.peek() {
        match c {

            c if c.is_whitespace() => {
                chars.next();
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            _ => return Err(format!("unexpected character '{c}'")),
        }
    }

    Ok(tokens)
}

// ============================================================================
// TESTS: run with `cargo test`
// These all fail (they hit the todo!) until tokenize is written.
// Make them pass. You can run a single test by name, e.g.
//   cargo test simple_addition
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*; // everything from the file above
    use Token::*; // lets tests write Num(3.0) instead of Token::Num(3.0)

    #[test]
    fn simple_addition() {
        assert_eq!(tokenize("3+4"), Ok(vec![Num(3.0), Plus, Num(4.0)]));
    }

    #[test]
    fn whitespace_decimals_and_parens() {
        assert_eq!(
            tokenize("  12.5 * (2 - 1)"),
            Ok(vec![Num(12.5), Star, LParen, Num(2.0), Minus, Num(1.0), RParen])
        );
    }

    #[test]
    fn every_operator() {
        assert_eq!(
            tokenize("+-*/()"),
            Ok(vec![Plus, Minus, Star, Slash, LParen, RParen])
        );
    }

    #[test]
    fn multi_digit_numbers_keep_their_first_digit() {
        // Catches the "vanishing first digit" bug.
        assert_eq!(tokenize("123"), Ok(vec![Num(123.0)]));
    }

    #[test]
    fn number_stops_at_the_next_token() {
        // Catches the inner loop eating a character that isn't part of the number.
        assert_eq!(tokenize("7*8"), Ok(vec![Num(7.0), Star, Num(8.0)]));
    }

    #[test]
    fn leading_and_trailing_dots() {
        assert_eq!(tokenize(".5 + 5."), Ok(vec![Num(0.5), Plus, Num(5.0)]));
    }

    #[test]
    fn empty_and_blank_input() {
        assert_eq!(tokenize(""), Ok(vec![]));
        assert_eq!(tokenize("   "), Ok(vec![]));
    }

    #[test]
    fn rejects_malformed_number() {
        assert!(tokenize("1..2").is_err());
    }

    #[test]
    fn rejects_lone_dot() {
        assert!(tokenize(".").is_err());
    }

    #[test]
    fn rejects_unknown_character() {
        assert!(tokenize("3 $ 4").is_err());
    }

    #[test]
    fn error_message_names_the_bad_character() {
        let err = tokenize("3 $ 4").unwrap_err();
        assert!(err.contains('$'), "message was: {err}");
    }

    #[test]
    fn lexer_does_not_check_grammar() {
        // Valid tokens in a nonsense order. The lexer only labels them.
        assert_eq!(tokenize(")3+"), Ok(vec![RParen, Num(3.0), Plus]));
    }
}