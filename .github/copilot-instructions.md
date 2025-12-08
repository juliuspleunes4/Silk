# Copilot Instructions for Silk Programming Language

## Project Overview
Silk is an open-source compiled programming language implemented in Rust. This is a compiler project, NOT an interpreter.

### Language Design Philosophy
- **Python-compatible syntax**: Silk uses identical syntax to Python for maximum ease of use and familiarity
- **C-level performance**: Compiled to native code for performance comparable to C/C++
- **Zero runtime overhead**: Ahead-of-time compilation eliminates interpreter overhead
- **Type inference**: Static typing with powerful type inference for safety without verbosity
- **Memory safety**: Leverages Rust's compiler implementation for safe memory management

### Language Features
- Python syntax (functions, classes, control flow, etc.)
- Static type system with gradual typing support
- Native compilation to machine code
- Zero-cost abstractions
- Compile-time optimization
- Interoperability with C libraries (ideally)
- No garbage collection overhead (compile-time memory management)

## Core Development Principles

### Architecture & Design
- **Scalability-first**: Design all components with future growth in mind
- **Clean structure**: Maintain well-organized repository structure
- **Incremental progress**: Break work into small, manageable tasks
- **Best practices**: Strictly follow Rust and compiler design best practices

### Code Quality Standards
- Prefer clear, maintainable code over clever solutions
- Write well-documented code using Rust doc comments (`///` for public items)
- Use Doxygen-style documentation where relevant
- Follow idiomatic Rust patterns and conventions
- Leverage Rust's type system for safety and correctness

### Testing Requirements
- **Extremely thorough tests** for EVERY feature
- Test edge cases and failure paths comprehensively
- If a test fails, **fix the implementation first**
- **NEVER modify tests just to make them pass**
- Maintain high test coverage across all compiler phases
- Write unit tests, integration tests, and end-to-end compiler tests

### Data & Placeholders
- **NO MOCK DATA** - absolutely never use mock or fake data
- Use TODO placeholders for unimplemented features
- Placeholders should be clear and descriptive
- Example: `// TODO: Implement type inference for generic functions`

### Documentation & Change Management
- Log ALL notable changes in `docs/CHANGELOG.md`
- Follow semantic versioning principles
- **Never create extra .md files** to summarize changes
- Keep documentation concise and in designated files

### Workflow
- Complete one small task at a time
- Stop and ask for the next task after completion
- Confirm understanding before starting implementation
- Report blockers or design decisions that need input

## Compiler Architecture Guidelines

### Expected Compiler Phases
1. **Lexical Analysis**: Tokenization of source code
2. **Parsing**: AST generation from tokens
3. **Semantic Analysis**: Type checking, symbol resolution
4. **Optimization**: IR transformations and optimizations
5. **Code Generation**: Target code emission

### Rust Project Structure
```
src/
├── main.rs           # CLI entry point
├── lib.rs            # Library root
├── lexer/            # Lexical analysis
├── parser/           # Syntax analysis
├── ast/              # Abstract syntax tree definitions
├── semantic/         # Type checking, symbol tables
├── codegen/          # Code generation
├── error/            # Error handling and reporting
└── utils/            # Shared utilities

tests/
├── unit/             # Unit tests
├── integration/      # Integration tests
└── fixtures/         # Test input files

docs/
├── CHANGELOG.md      # Change log
└── TODO.md           # Task tracking
```

### Code Style
- Use `rustfmt` for consistent formatting
- Run `clippy` and address all warnings
- Keep functions small and focused (single responsibility)
- Use meaningful variable and function names
- Prefer pattern matching over nested conditionals
- Use `Result<T, E>` for error handling

### Error Handling
- Provide clear, helpful error messages
- Include source location information (line, column)
- Use custom error types with proper context
- Never use `.unwrap()` in production code
- Prefer `?` operator for error propagation

## Git Commit Guidelines
- Use conventional commit messages
- Prefix: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`
- Keep commits atomic and focused
- Reference issues when applicable

## Response Protocol
When given a task:
1. Confirm understanding of requirements
2. Implement the solution incrementally
3. Write comprehensive tests
4. Update CHANGELOG.md with changes
5. Report completion and wait for next instruction

**Remember**: Quality over speed. Thorough testing and clean code are paramount.
