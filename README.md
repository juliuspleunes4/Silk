# Silk Programming Language

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](Cargo.toml)
[![Tests](https://img.shields.io/badge/tests-1078%20passing-brightgreen.svg)](#testing)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)

> A compiled programming language with Python-compatible syntax and C-level performance

## Overview

Silk is an open-source compiled programming language that combines the ease and familiarity of Python syntax with the performance of native compiled code. Built in Rust, Silk aims to deliver zero-overhead execution while maintaining the developer-friendly experience Python users love.

### Key Features

- **Python-Compatible Syntax** — Write code using familiar Python syntax
- **Native Compilation** — Compiles to native machine code for maximum performance
- **Static Type System** — Optional type annotations with powerful type inference
- **Memory Safety** — Leverages Rust's compiler implementation for safe memory management
- **Zero Runtime Overhead** — No interpreter or garbage collector overhead
- **Modern Tooling** — Built-in CLI tools for building, running, and analyzing code

## Philosophy

Silk is designed around three core principles:

1. **Familiarity** — Use Python's clean, readable syntax
2. **Performance** — Achieve C/C++ level execution speeds through ahead-of-time compilation
3. **Safety** — Provide compile-time guarantees for memory safety and type correctness

## Installation

### From Source

```bash
git clone https://github.com/juliuspleunes4/silk.git
cd silk
cargo build --release
```

The compiled binary will be available at `target/release/silk`.

## Usage

```bash
# Build a Silk program
silk build program.silk

# Run a Silk program
silk run program.silk

# Check syntax without building
silk check program.silk

# Tokenize source code (debugging)
silk lex program.silk
```

## Example Code

```python
# Variables and basic types
name = "Alice"
age = 30
is_active = True

# Functions
def greet(name: str) -> str:
    return f"Hello, {name}!"

# Control flow
if age >= 18:
    print("Adult")
else:
    print("Minor")

# Loops
for i in range(10):
    print(i)

# Collections
numbers = [1, 2, 3, 4, 5]
config = {"host": "localhost", "port": 8080}
unique_ids = {1, 2, 3}

# Classes
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
    
    def distance(self) -> float:
        return (self.x ** 2 + self.y ** 2) ** 0.5
```

## Project Structure

Silk is organized as a Cargo workspace with multiple crates:

- **silk-cli** — Command-line interface and tools
- **silk-lexer** — Lexical analysis and tokenization
- **silk-parser** — Syntax analysis and AST generation
- **silk-ast** — Abstract syntax tree definitions
- **silk-compiler** — Compilation orchestration
- **silk-semantic** — Symbol table and semantic analysis (Phase 2 in progress)
- **silk-codegen** _(planned)_ — Native code generation

## Development Status

Silk is in active development. The compiler frontend (lexer and parser) is complete, and Phase 2 (semantic analysis) is now in progress.

### Current Capabilities

- ✅ Full lexical analysis with Python-style indentation
- ✅ Statement parsing (functions, classes, control flow, imports, etc.)
- ✅ Expression parsing (operators, literals, calls, comprehensions, etc.)
- ✅ Type annotation parsing
- 🚀 Semantic analysis (symbol table, name resolution, scope management)
- ⏳ Code generation (planned)

See [docs/TODO.md](docs/TODO.md) for detailed progress tracking.

## Testing

Silk has a comprehensive test suite to ensure correctness:

```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test --package silk-lexer
cargo test --package silk-parser
cargo test --package silk-semantic
```

**Current test coverage:** 690 tests across all components (13 ignored)
- 126 lexer tests (11 unit + 115 integration)
- 264 parser tests (9 annotated assignment + 255 comprehensive)
- 8 type system unit tests
- 292 semantic analysis tests:
  - 31 analyzer tests (3 ignored)
  - 9 annotated assignment tests
  - 41 binary operation tests (10 ignored)
  - 19 function call type inference tests
  - 12 collection type system tests
  - 24 decorators/bases/keywords tests
  - 11 dict type inference tests
  - 14 forward reference tests
  - 4 function type storage tests
  - 13 list type inference tests
  - 44 name resolution tests
  - 6 parameter defaults tests
  - 8 set type inference tests
  - 17 symbol table tests
  - 11 tuple type inference tests
  - 28 type inference tests

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

### Development Guidelines

- Follow Rust idioms and best practices
- Write tests for new features
- Update documentation as needed
- Keep commits focused and well-described

See [.github/copilot-instructions.md](.github/copilot-instructions.md) for detailed development principles.

## Documentation

- [Architecture Overview](docs/ARCHITECTURE.md) — Compiler design and implementation details
- [Development Steps](docs/STEPS.md) — Detailed implementation roadmap
- [Known Limitations](docs/KNOWN_LIMITATIONS.md) — Current limitations with resolution plans
- [TODO List](docs/TODO.md) — Upcoming features and tasks
- [Changelog](docs/CHANGELOG.md) — History of changes and releases

## Known Limitations

Silk is under active development. Some known limitations include:

- **Lambda parameter defaults** — Not yet supported by parser (semantic analyzer ready)
- **Method call tracking** — Method calls (`obj.method()`) not tracked in control flow analysis
- **Decorator tracking** — Decorator functions not marked as used in control flow analysis
- **Generic type constraints** — `List[int]` parsed but constraints not enforced
- **Code generation** — Not yet implemented (analysis-only currently)

For detailed information and implementation plans, see [docs/KNOWN_LIMITATIONS.md](docs/KNOWN_LIMITATIONS.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

Silk is inspired by Python's elegant syntax and Rust's focus on safety and performance. Special thanks to the Rust community for providing excellent tools and libraries.

---

**Note:** Silk is experimental software under active development. APIs and language features may change as the project evolves.
