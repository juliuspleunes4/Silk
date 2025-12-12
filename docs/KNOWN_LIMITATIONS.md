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

### 4. Control Flow Analysis - Exception Handling Edge Cases

**Status**: 🔄 **IN PROGRESS** (December 12, 2025)

**Description**: Control flow analysis has several edge cases related to exception handling that are not fully tracked, leading to code being marked as reachable when it might not be.

**Implementation Plan**:

This is divided into 3 incremental tasks, each with comprehensive testing:

#### Task 1: Track Bare `raise` as Diverging Control Flow ✅ **COMPLETE**
**Difficulty**: Easy-Medium  
**File**: `crates/silk-semantic/src/control_flow.rs`  
**Tests**: 8 tests in `test_bare_raise_divergence.rs`  
**Completed**: December 12, 2025

**Previous Behavior**: Bare `raise` statements (re-raising exceptions) didn't mark subsequent code as unreachable.

**Implementation**:
- Verified `StatementKind::Raise` correctly sets `is_reachable = false`
- Verified try/except reachability logic: `after_except_reachable = try_reachable || handlers_reachable`
- Key insight: If try can complete normally (no exception), code after try/except is reachable even if all except handlers raise

**Test Coverage**:
- ✅ Bare raise in except handler → code after unreachable
- ✅ Bare raise with finally block → code after reachable if try succeeds
- ✅ Bare raise vs `raise Exception()` (with expression)
- ✅ Try returns + all except handlers raise → code after unreachable
- ✅ Try raises + except raises → code after unreachable
- ✅ Module-level raise → code after unreachable
- ✅ All except handlers raise but try can succeed → code after reachable
- ✅ Single except without raise → code after reachable

**Bug Fixes**:
- Fixed test helper to properly capture errors from `analyze()` method

---

#### Task 2: Improve Try/Except Return Path Analysis ✅ **COMPLETE**
**Difficulty**: Medium  
**File**: `crates/silk-semantic/src/control_flow.rs`  
**Tests**: 12 tests in `test_try_except_return_paths.rs`
**Completed**: December 12, 2025

**Previous Behavior**: When try block returns but except doesn't, code after try/except was incorrectly marked as reachable. Finally blocks didn't properly combine reachability.

**Implementation**:
- Fixed finally block reachability logic to combine both try/except/else AND finally reachability
- Key change: `self.is_reachable = after_try_except_else_reachable && self.is_reachable;`
- This ensures code after is unreachable if EITHER try/except/else OR finally always diverges
- Existing try/except logic (`try_reachable || handlers_reachable`) was already correct

**Test Coverage**:
- ✅ Try returns, except doesn't → reachable after
- ✅ Try returns, all excepts return → unreachable after
- ✅ Try returns, some excepts return → reachable after
- ✅ Try doesn't return, except returns → reachable after
- ✅ Else clause: both except and else return → unreachable after
- ✅ Try returns with else (else unreachable) → detected
- ✅ Finally with cleanup (doesn't return) → preserves try/except reachability
- ✅ Finally with return → overrides, makes unreachable after
- ✅ Nested try/except with returns → all paths tracked
- ✅ Multiple except handlers with mixed behavior → correctly handled
- ✅ Bare except returning → catches everything, unreachable after
- ✅ Try raises, except returns → unreachable after

**Bug Fixes**:
- Fixed finally block incorrectly making code reachable when try/except all returned

---

#### Task 3: Track All-Paths-Return Across Try/Except in Conditionals
**Difficulty**: Medium-Hard  
**File**: `crates/silk-semantic/src/control_flow.rs`  
**Tests**: 5-7 tests

**Current Behavior**: When try/except blocks are inside conditionals, analyzer doesn't track whether all code paths return across the try blocks.

**Implementation Steps**:
1. When analyzing if/else statements, track whether each branch returns
2. For branches containing try/except: consider the branch returns if try/except analysis says unreachable after
3. Function returns on all paths if all top-level branches return
4. Handle nested conditionals within try/except blocks

**Test Cases**:
- If with try/except that always returns in try/except → branch returns
- If/else where both branches have try/except that return → function returns all paths
- Try/except in if, regular return in else → mixed behavior
- Nested if within try block
- Try/except in while loop condition branch

**Success Criteria**: All-paths-return correctly tracked through conditional branches containing try/except

---

**Overall Test Count**: 16-22 new tests total

---

**Current Status Summary**:
- ⏳ Task 1: Bare `raise` divergence - Not started
- ⏳ Task 2: Try/except return path analysis - Not started  
- ⏳ Task 3: All-paths-return in conditionals - Not started

---

**Original Sub-limitations (for reference)**:

#### 4.1 Bare `raise` Statements Not Tracked as Diverging

**Impact**: Code after a bare `raise` statement (re-raising the current exception) is marked as reachable instead of unreachable.

**Example**:
```python
def foo():
    try:
        operation()
    except Exception as e:
        log(e)
        raise  # Re-raises the exception
    print("marked as reachable")  # Should be unreachable
```

**Workaround**: None currently. Bare `raise` is valid Python but not tracked as diverging control flow.

**Test**: See `test_except_with_bare_raise` in `test_complex_exception_patterns.rs`

#### 4.2 Try Block with Return + Except Without Return

**Impact**: When a try block returns but the except handler doesn't, code after the try/except/finally is incorrectly marked as reachable.

**Example**:
```python
def foo():
    try:
        return "success"
    except Exception:
        handle()  # Doesn't return
    finally:
        cleanup()
    print("marked as reachable")  # Might be unreachable if no exception
```

**Workaround**: Ensure all exception handlers also return if the try block returns.

**Test**: See `test_return_in_try_with_finally_no_return` in `test_complex_exception_patterns.rs`

#### 4.3 Try/Except in Conditionals - All-Paths-Return Not Tracked

**Impact**: When try/except blocks are inside conditionals, the analyzer doesn't fully track whether all code paths return across the try blocks.

**Example**:
```python
def foo(flag):
    if flag:
        try:
            operation()
        except Exception:
            return "error"
    else:
        return "skipped"
    print("marked as reachable")  # Might be reachable if try completes normally
```

**Workaround**: Explicitly handle all return paths in try/except blocks.

**Test**: See `test_try_except_in_conditional` in `test_complex_exception_patterns.rs`

**Implementation Plan**: These are edge cases in control flow analysis that would require more sophisticated exception path tracking. Future enhancement to track:
- Bare raise as diverging control flow
- Exception handler return path analysis
- Cross-block all-paths-return tracking

**Effort**: Medium to Hard

---

## Summary

| # | Limitation | Priority | Difficulty | Status |
|---|------------|----------|------------|--------|
| 1 | Generic type constraints | Low | Hard | Future enhancement |
| 2 | Type narrowing | Low | Hard | Future enhancement |
| 3 | Code generation | High | Very Hard | Phase 7+ |
| 4 | Control flow - exception edge cases | Low | Medium-Hard | Known behavior |

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