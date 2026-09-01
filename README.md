# mylexer

A simple hand-written lexer for arithmetic expressions written in Rust.

## Supported tokens

- Number (integer literals)
- Plus (`+`), Minus (`-`), Star (`*`), Slash (`/`)
- Open parenthesis `(`, close parenthesis `)`

Whitespace is ignored.

## Example

The input `2*4+(5-3)` produces:

```
Number("2"), Star, Number("4"), Plus,
OpenParenthesis, Number("5"), Minus, Number("3"), CloseParenthesis
```

## Structure

- `src/token.rs` - token types and the `Token` struct
- `src/lexer.rs` - the lexer that converts input into a list of tokens

## Usage

```bash
cargo build
cargo run
```
