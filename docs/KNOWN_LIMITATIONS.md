# Known Limitations

This document tracks known limitations in the Silk compiler, their impact, and implementation status.

---

## Table of Contents
1. [Active Limitations](#active-limitations)
2. [Resolved Limitations](#resolved-limitations)

---

## Active Limitations

### 1. Generic Type Constraints Not Enforced

**Status**: ⚠️ Future enhancement

**Description**: Generic types like `List[int]` are parsed and stored but constraints are not enforced during type checking.

**Impact**:
- `List[int]` is treated the same as `List[str]` or just `List`
- No error for appending wrong type to typed list
- Reduces type safety for generic collections

**Example**:
```python
numbers: List[int] = [1, 2, 3]
numbers.append("string")  # Should error, but doesn't
```

**Implementation Plan**: TBD (requires generic type system enhancement)

**Effort**: Hard

---

### 2. Type Narrowing Not Supported

**Status**: ⚠️ Future enhancement

**Description**: Type narrowing from runtime checks (e.g., `isinstance`, `is None` checks) is not performed.

**Impact**:
- Optional types (`Optional[T]` or `T | None`) are not narrowed after None check
- Type guards don't affect type inference
- May require unnecessary type assertions

**Example**:
```python
def process(value: Optional[str]) -> str:
    if value is None:
        return "default"
    # value is still Optional[str] here, should be str
    return value.upper()  # Might warn about None
```

**Implementation Plan**: TBD (requires control flow sensitive typing)

**Effort**: Hard

---

### 3. No Code Generation Yet

**Status**: ⚠️ Not implemented

**Description**: The compiler currently only performs lexical analysis, parsing, and semantic analysis. Code generation is not yet implemented.

**Impact**:
- Cannot compile to executable code
- Cannot run Silk programs
- Project is currently an analysis-only tool

**Implementation Plan**:

This is a major Phase 7+ effort requiring:
1. IR (Intermediate Representation) design
2. Backend selection (LLVM, Cranelift, or custom)
3. Code generation for all AST nodes
4. Runtime library implementation
5. Standard library implementation
6. Optimization passes

**See**: `docs/TODO.md` Phase 7 and beyond

**Effort**: Very Hard

---

## Summary

| # | Limitation | Priority | Difficulty | Status |
|---|------------|----------|------------|--------|
| 1 | Generic type constraints | Low | Hard | Future enhancement |
| 2 | Type narrowing | Low | Hard | Future enhancement |
| 3 | Code generation | High | Very Hard | Phase 7+ |

---

## Resolved Limitations

### ✅ Lambda Parameter Defaults (Resolved December 12, 2025)

**Previous Issue**: Lambda expressions did not support default parameter values, unlike regular function definitions.

**Example of Previously Broken Code**:
```python
f = lambda x=10: x * 2  # Would fail to parse
multiplier = lambda a, b=2: a * b  # Would fail
```

**Solution**: Extended lambda parameter parsing to handle default values with full Python semantics.

**Implementation**:
- Modified `parse_lambda()` in `crates/silk-parser/src/expr.rs`
- Added support for `= <expr>` after parameter names
- Enforces non-default after default rule (raises `NonDefaultParamAfterDefault` error)
- Handles trailing commas correctly
- Default values evaluated as expressions (any complexity supported)

**Impact**: Lambda expressions now have feature parity with function definitions for parameters

**Tests Added**: 
- 11 parser tests in `test_lambda_defaults.rs`
- 4 semantic tests for type/scope checking
- Total: 15 new passing tests

**See**: CHANGELOG.md for full details

---

### ✅ Inline Comment Support (Resolved December 12, 2025)

**Previous Issue**: The parser could not handle inline comments (comments after code on the same line). Comments had to be on their own lines.

**Example of Previously Broken Code**:
```python
x = 10  # this would cause parser errors
```

**Solution**: Modified lexer to skip inline comments as whitespace rather than tokenizing them.

**Implementation**:
- Updated `skip_whitespace_inline()` in `lexer.rs`
- Inline comments consumed up to end of line
- Standalone comments still generate `Comment` tokens (for documentation tools)
- Line/column tracking preserved

**Impact**: Enables idiomatic Python code with inline documentation

**Tests Added**: 19 comprehensive tests in `test_inline_comments.rs`

**See**: CHANGELOG.md for full details

---

### ✅ Global/Nonlocal Statement Support (Resolved December 12, 2025)

**Previous Issue**: Variables declared with `global` or `nonlocal` were not marked as initialized in control flow analysis, causing false "uninitialized variable" warnings.

**Example of Previously Broken Code**:
```python
counter = 0
def increment():
    global counter  # Would trigger false "uninitialized" warning
    counter = counter + 1
```

**Solution**: Enhanced control flow analyzer to recognize global/nonlocal statements and mark referenced variables as initialized.

**Implementation**:
- Added handling in `control_flow.rs` for `StatementKind::Global` and `StatementKind::Nonlocal`
- Variables in global/nonlocal statements automatically marked as initialized
- Works with multiple variables and nested functions

**Impact**: Eliminates false positive warnings for legitimate global/nonlocal usage

**Tests Added**: 18 comprehensive tests in `test_global_nonlocal.rs`

**See**: CHANGELOG.md for full details

---

### ✅ Decorator Functions Not Tracked as Used (Resolved December 12, 2025)

**Previous Issue**: When a function was used as a decorator (e.g., `@decorator`), the decorator function itself was not tracked as being called/used, resulting in false "unused function" warnings.

**Example of Previously Broken Code**:
```python
def my_decorator(func):  # Would be marked as unused
    def wrapper(*args, **kwargs):
        result = func(*args, **kwargs)
        return result
    return wrapper

@my_decorator  # This usage was not tracked
def greet(name):
    return f"Hello, {name}"

greet("World")  # Only this call was tracked
```

**Solution**: Enhanced control flow analyzer to track decorator usage for both function and class decorators.

**Implementation**:
- Added `track_decorator_usage()` method in `crates/silk-semantic/src/control_flow.rs`
- Integrated into `FunctionDef` and `ClassDef` statement handling
- Handles simple decorators, decorators with arguments, and decorator chains

**Decorator Patterns Now Supported**:
- Simple decorators: `@decorator`
- Decorators with arguments: `@decorator(arg1, arg2)`
- Decorator chains: `@dec1 @dec2 @dec3`
- Class decorators: `@class_decorator class MyClass: pass`

**Impact**: Eliminates false "unused function" warnings for decorator functions

**Tests Added**: 10 comprehensive tests in `test_decorator_tracking.rs`

**See**: CHANGELOG.md for full details

---

### ✅ Method Calls Not Tracked as Function Usage (Resolved December 12, 2025)

**Previous Issue**: Method calls using attribute access syntax (`obj.method()`) were not tracked as function calls in control flow analysis, causing instance methods to be incorrectly reported as unused functions.

**Solution**: Implemented `track_all_calls_in_expression()` method in `control_flow.rs` that recursively tracks all function/method calls in expressions.

**Implementation Details**:
- Handles `Expression::Identifier` for direct calls
- Handles `Expression::Attribute` for method calls via attribute access
- Recursively processes `Expression::Call` to handle chained method calls
- Integrated into `Expression::Call` handling

**Impact**: Method calls are now properly tracked

**Tests Added**: 10 comprehensive tests in `test_method_call_tracking.rs`

**See**: CHANGELOG.md for full details

---

### ✅ Type Inference for Comprehensions (Resolved December 12, 2025)

**Previous Issue**: Type inference for comprehensions was not implemented. All comprehensions returned `Unknown` type regardless of element/key/value types.

**Solution**: Implemented full type inference for list/set/dict comprehensions.

**Implementation**:
- List comprehensions: `[expr for x in iterable]` → `List[expr_type]`
- Set comprehensions: `{expr for x in iterable}` → `Set[expr_type]`
- Dict comprehensions: `{k: v for x in iterable}` → `Dict[k_type, v_type]`
- Generator variables are typed based on iterable element types
- Comprehensions create their own scopes (Python 3 semantics)

**Example**:
```python
numbers: List[int] = [1, 2, 3]
doubled = [x * 2 for x in numbers]  # ✅ Now: List[int]

evens = [x for x in numbers if x % 2 == 0]  # ✅ Now: List[int]

word_lengths = {word: len(word) for word in ["hello", "world"]}  # ✅ Now: Dict[str, int]
```

**Implementation Details**:
- Modified `infer_type()` to handle ListComp, SetComp, DictComp
- Added `extract_iterable_element_type()` helper method
- Fixed case sensitivity in type annotation resolution (`List` vs `list`)

**Tests Added**: 19 comprehensive tests in `test_comprehension_type_inference.rs`

**See**: CHANGELOG.md for full details

---

### ✅ Complex Control Flow Patterns Testing (Resolved December 12, 2025)

**Previous Issue**: Many edge cases and complex exception handling patterns were not explicitly tested, leaving uncertainty about analyzer correctness.

**Solution**: Created comprehensive test suite with 15 tests covering previously untested complex patterns.

**Patterns Now Tested**:
- Break and continue in finally blocks
- Return in except handlers with else clauses
- Multiple exception types in single handler
- Deeply nested try blocks (3+ levels)
- Try/except within conditionals
- Bare raise statements
- Nested finally blocks
- Complete try/except/else/finally combinations
- Exception handling within exception handlers
- Try/except in loops with break/continue

**Tests Added**: 15 tests in `test_complex_exception_patterns.rs`

**Impact**: Significantly increased confidence in exception handling analysis

**See**: CHANGELOG.md for full details

---

## Contributing

When adding a new limitation to this document:

1. **Describe the limitation clearly** with examples
2. **Explain the impact** on users
3. **Provide a workaround** if available
4. **Estimate the effort** (Easy/Medium/Hard/Very Hard)
5. **Update the summary table**

When a limitation is resolved:
- Move it to the "Resolved Limitations" section
- Update CHANGELOG.md with full implementation details
- Update the summary table

---

*Last Updated: December 12, 2025*
