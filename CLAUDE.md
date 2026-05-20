# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ROX is a tree-walk interpreter for a dynamically-typed scripting language, built in Rust following the [Crafting Interpreters](https://craftinginterpreters.com/) book. It supports a REPL and file execution modes.

## Build & Run Commands

```bash
# Build
cargo build               # debug binary at target/debug/rox
cargo build --release     # optimized binary

# Test
cargo test                # runs all unit tests
cargo test <test_name>    # run a single test

# Run
./rox                     # start REPL
./rox examples/hello.rox  # execute a .rox file
```

Initial setup creates a symlink: `ln -s target/debug/rox rox`

## Architecture

The interpreter follows the classic pipeline: **source → tokens → AST → execution**

```
Scanner → Parser → AST → Interpreter
                          ↕
                      Environment
```

- **`scanner.rs`** — Lexer: converts source text into a flat `Vec<Token>`
- **`parser.rs`** — Recursive descent parser: consumes tokens and builds an AST of `Expr` / `Stmt` nodes
- **`ast.rs`** — Node type definitions: `Expr` (expressions), `Stmt` (statements), and `Token`/`TokenType` enums
- **`interpreter.rs`** — Tree-walk evaluator: implements the `Visitor` trait to evaluate expressions and execute statements
- **`environment.rs`** — Lexical scoping via a stack of `HashMap<String, Value>` scopes
- **`visitor.rs`** — Visitor trait interface shared by the interpreter and printer
- **`printer.rs`** — Debug utility: renders AST as S-expressions (e.g. `(+ 2 2)`)
- **`main.rs`** — Entry point via `clap`: dispatches to REPL or file execution

### Key Design Patterns

**Visitor pattern:** `Visitor` trait in `visitor.rs` has methods for every AST node type. Both `Interpreter` and `Printer` implement it, keeping traversal logic separate from data structures.

**For loops as syntax sugar:** `for` loops are desugared into `while` loops in the parser — there is no `ForStmt` in the AST.

**Scoping:** `Environment` maintains a `Vec` of scopes. Entering a block pushes a new scope; exiting pops it. Variable lookup walks the stack from innermost to outermost.

## ROX Language Features

- Literals: numbers, strings, booleans, `nil`
- Operators: `+`, `-`, `*`, `/`, `>`, `>=`, `<`, `<=`, `==`, `!=`, `!`, `and`, `or`
- Statements: `var`, `print`, `if`/`else`, `while`, `for`, block `{}`
- Dynamic typing with implicit string/number coercion in some operations
