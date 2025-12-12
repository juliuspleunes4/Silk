# Known Limitations

This document tracks known limitations in the Silk compiler, their impact, and step-by-step plans for resolution.

---

## Table of Contents
1. [Parser Limitations](#parser-limitations)
2. [Semantic Analysis Limitations](#semantic-analysis-limitations)
3. [Control Flow Analysis Limitations](#control-flow-analysis-limitations)

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
- All statement types with inline comments
- Edge cases (end of file, empty comments, etc.)
- Hash symbols in strings vs. comments
- Line number preservation

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

**Previous Status**: ⚠️ Known limitation, documented

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
- Processes decorator arguments for variable usage

**Decorator Patterns Now Supported**:
- Simple decorators: `@decorator`
- Decorators with arguments: `@decorator(arg1, arg2)`
- Decorator chains: `@dec1 @dec2 @dec3`
- Class decorators: `@class_decorator class MyClass: pass`
- Decorators with variables: `@decorator(config_value)`
- Decorators with expressions: `@decorator(x * 2 + 5)`
- Decorator attribute access: `@module.decorator` (partially)

**Impact**: Eliminates false "unused function" warnings for decorator functions

**Tests Added**: 10 comprehensive tests in `test_decorator_tracking.rs`
- Simple decorator usage
- Decorators with positional/keyword arguments
- Multiple decorators (chains)
- Class decorators
- Variable and expression arguments
- Nested decorator factories
- Unused decorator detection (negative test)

**Modified Tests**: Updated `test_decorator_with_control_flow` in `test_control_flow_integration.rs`

**See**: CHANGELOG.md for full details

---

## Parser Limitations

Currently no known parser limitations. All planned parser features have been implemented.

---

## Semantic Analysis Limitations

### 1. Type Inference for Complex Comprehensions

**Status**: ⚠️ Partial support

**Description**: Type inference for nested comprehensions and comprehensions with complex conditions may be incomplete.

**Impact**:
- Nested comprehensions might not infer the most specific type
- Complex filtering conditions may result in `Unknown` type

**Example**:
```python
# May not fully infer type
nested = [[x * 2 for x in row] for row in matrix]

# Filter conditions might lose type information
evens = [x for x in numbers if x % 2 == 0]  # Type might be List[Unknown]
```

**Implementation Plan**: TBD (requires type system enhancement)

---

## Control Flow Analysis Limitations

### 2. Method Calls Not Tracked as Function Usage

**Status**: ✅ **RESOLVED** (December 12, 2025)

**Previous Issue**: Method calls using attribute access syntax (`obj.method()`) were not tracked as function calls in control flow analysis. This caused instance methods to be incorrectly reported as unused functions.

**Solution**: Implemented `track_all_calls_in_expression()` method in `control_flow.rs` that recursively tracks all function/method calls in expressions.

**Implementation Details**:
- Handles `Expression::Identifier` for direct calls
- Handles `Expression::Attribute` for method calls via attribute access
- Recursively processes `Expression::Call` to handle chained method calls
- Does not check initialization to avoid false positives for class names
- Integrated into `Expression::Call` handling

**Testing**: Covered by 10 comprehensive tests in `test_method_call_tracking.rs` including:
- Simple method calls
- Multiple methods on same object
- Chained method calls (`obj.m1().m2()`)
- Method calls in conditionals and loops
- Proper unused function detection when methods are truly unused

**Example Now Working**:
```python
class Calculator:
    def add(self, a, b):  # No longer marked as unused
        return a + b

calc = Calculator()
result = calc.add(5, 3)  # This call is now tracked
```

**Current Documentation**: 
- Documented in `test_class_methods_control_flow` integration test
- Explicitly marked as expected behavior

**Implementation Plan**:

#### Step 1: Extend Call Tracking in Control Flow Analyzer
**File**: `crates/silk-semantic/src/control_flow.rs`

**Current State**:
```rust
Expression::Call { func, .. } => {
    if let Expression::Name { id, .. } = &**func {
        self.mark_used(id);
    }
}
```

**Target State**:
```rust
Expression::Call { func, .. } => {
    match &**func {
        // Direct function call: func()
        Expression::Name { id, .. } => {
            self.mark_used(id);
        }
        // Method call: obj.method()
        Expression::Attribute { attr, .. } => {
            // Mark the method name as used
            self.mark_used(attr);
            // Future: track through class definitions to find actual function
        }
        _ => {}
    }
}
```

#### Step 2: Track Method Definitions in Classes
**File**: `crates/silk-semantic/src/control_flow.rs`

**New Data Structure**:
```rust
struct ControlFlowAnalyzer {
    // ... existing fields ...
    
    /// Maps class names to their method names
    class_methods: HashMap<String, HashSet<String>>,
    
    /// Maps method names to their defining class (for reverse lookup)
    method_to_class: HashMap<String, String>,
}
```

**Logic**:
```rust
fn analyze_class_def(&mut self, name: &str, body: &[Statement]) {
    let mut methods = HashSet::new();
    
    for stmt in body {
        if let Statement::FunctionDef { name: method_name, .. } = stmt {
            methods.insert(method_name.clone());
            self.method_to_class.insert(method_name.clone(), name.to_string());
        }
    }
    
    self.class_methods.insert(name.to_string(), methods);
}
```

#### Step 3: Link Method Calls to Method Definitions
**File**: `crates/silk-semantic/src/control_flow.rs`

**Enhanced Call Tracking**:
```rust
Expression::Call { func, .. } => {
    match &**func {
        Expression::Name { id, .. } => {
            self.mark_used(id);
        }
        Expression::Attribute { value, attr, .. } => {
            // Try to determine the class of the object
            if let Some(class_name) = self.infer_class_from_expression(value) {
                // Check if this class has this method
                if let Some(methods) = self.class_methods.get(&class_name) {
                    if methods.contains(attr) {
                        // Mark the actual method function as used
                        if let Some(defining_class) = self.method_to_class.get(attr) {
                            // Mark as used in context of class
                            self.mark_method_used(defining_class, attr);
                        }
                    }
                }
            } else {
                // Can't determine class, conservatively mark method name as used
                self.mark_used(attr);
            }
        }
        _ => {}
    }
}
```

#### Step 4: Create Test Suite
**File**: `crates/silk-semantic/tests/test_method_call_tracking.rs` (new file)

**Test Cases**:

1. **Basic method call tracking**
   ```python
   class Math:
       def double(self, x):
           return x * 2
   
   m = Math()
   result = m.double(5)  # double should NOT be marked unused
   ```

2. **Multiple methods, some unused**
   ```python
   class Calculator:
       def add(self, a, b):
           return a + b
       
       def subtract(self, a, b):  # This should be marked unused
           return a - b
   
   calc = Calculator()
   result = calc.add(5, 3)
   ```

3. **Method calls in different scopes**
   ```python
   class Processor:
       def process(self, data):
           return data.upper()
   
   def main():
       p = Processor()
       result = p.process("hello")
   
   main()
   ```

4. **Chained method calls**
   ```python
   class Builder:
       def set_x(self, x):
           self.x = x
           return self
       
       def set_y(self, y):
           self.y = y
           return self
       
       def build(self):
           return (self.x, self.y)
   
   result = Builder().set_x(10).set_y(20).build()
   # None of these should be marked unused
   ```

5. **Method stored in variable**
   ```python
   class Handler:
       def handle(self, event):
           return event
   
   h = Handler()
   handler_func = h.handle
   result = handler_func("click")  # May not track this
   ```

6. **Inherited methods**
   ```python
   class Base:
       def base_method(self):
           return "base"
   
   class Derived(Base):
       def derived_method(self):
           return "derived"
   
   d = Derived()
   d.base_method()  # Should track if inheritance is supported
   d.derived_method()
   ```

**Test Count**: 6-10 tests

#### Step 5: Update Integration Tests
**File**: `crates/silk-semantic/tests/test_integration.rs`

- Remove or update `test_class_methods_control_flow` to reflect new behavior
- Add tests showing method tracking works correctly

**Completion Criteria**:
- [ ] Control flow analyzer tracks attribute access in calls
- [ ] Class methods tracked separately from functions
- [ ] Method calls properly mark methods as used
- [ ] All new tests pass
- [ ] Integration tests updated
- [ ] Update CHANGELOG.md
- [ ] Update TODO.md

**Estimated Effort**: Medium (2-3 implementation sessions)

---

### ✅ Decorator Functions Not Tracked as Used (Resolved December 12, 2025)

**Previous Status**: ⚠️ Known limitation, documented

**Description**: When a function is used as a decorator (e.g., `@decorator`), the decorator function itself is not tracked as being called/used. However, the decorated function is correctly marked as used.

**Impact**:
- Decorator functions will be flagged as `UnusedFunction` even though they are applied
- Affects code quality warnings, not correctness
- Decorated functions are correctly handled

**Example**:
```python
def my_decorator(func):  # Will be marked as unused
    def wrapper(*args, **kwargs):
        print("Before")
        result = func(*args, **kwargs)
        print("After")
        return result
    return wrapper

@my_decorator  # This usage is not tracked
def greet(name):  # Correctly marked as used if called
    return f"Hello, {name}"

greet("World")  # This call is tracked
```

**Current Documentation**:
- Documented in `test_decorator_with_control_flow` integration test
- Explicitly marked as expected behavior

**Implementation Plan**:

#### Step 1: Track Decorator Applications
**File**: `crates/silk-semantic/src/control_flow.rs`

**Current State**:
```rust
Statement::FunctionDef { name, decorators, .. } => {
    self.define(name);
    // decorators are not processed for usage
}
```

**Target State**:
```rust
Statement::FunctionDef { name, decorators, .. } => {
    // Track decorator usage
    for decorator in decorators {
        self.mark_decorator_used(decorator);
    }
    
    self.define(name);
    // ... rest of function analysis
}
```

#### Step 2: Implement Decorator Usage Marking
**File**: `crates/silk-semantic/src/control_flow.rs`

**New Method**:
```rust
fn mark_decorator_used(&mut self, decorator: &Expression) {
    match decorator {
        // Simple decorator: @decorator_name
        Expression::Name { id, .. } => {
            self.mark_used(id);
        }
        
        // Decorator with arguments: @decorator(arg1, arg2)
        Expression::Call { func, args, .. } => {
            // Mark the decorator function as used
            if let Expression::Name { id, .. } = &**func {
                self.mark_used(id);
            }
            
            // Also analyze arguments for any names used
            for arg in args {
                self.analyze_expression_for_usage(arg);
            }
        }
        
        // Decorator from module: @module.decorator
        Expression::Attribute { attr, value, .. } => {
            // Mark the attribute name (decorator) as used
            self.mark_used(attr);
            
            // Also analyze the base expression
            self.analyze_expression_for_usage(value);
        }
        
        _ => {
            // Complex decorator expression, analyze for any usage
            self.analyze_expression_for_usage(decorator);
        }
    }
}
```

#### Step 3: Handle Decorator Chains
**File**: `crates/silk-semantic/src/control_flow.rs`

**Support Multiple Decorators**:
```python
@decorator1
@decorator2(arg="value")
@module.decorator3
def my_function():
    pass
```

All three decorators (`decorator1`, `decorator2`, `decorator3`) should be marked as used.

#### Step 4: Create Test Suite
**File**: `crates/silk-semantic/tests/test_decorator_tracking.rs` (new file)

**Test Cases**:

1. **Simple decorator**
   ```python
   def my_decorator(func):
       return func
   
   @my_decorator  # Should be marked as used
   def greet():
       return "Hello"
   
   greet()
   ```
   Expected: No unused function warnings

2. **Decorator with arguments**
   ```python
   def parametrized_decorator(param):
       def decorator(func):
           return func
       return decorator
   
   @parametrized_decorator("config")  # Should be marked as used
   def process():
       return "Processing"
   
   process()
   ```
   Expected: No unused function warnings

3. **Multiple decorators**
   ```python
   def decorator1(func):
       return func
   
   def decorator2(func):
       return func
   
   @decorator1  # Should be marked as used
   @decorator2  # Should be marked as used
   def multi_decorated():
       return "Decorated"
   
   multi_decorated()
   ```
   Expected: No unused function warnings

4. **Unused decorator (actually unused)**
   ```python
   def used_decorator(func):
       return func
   
   def unused_decorator(func):  # Should be marked as unused
       return func
   
   @used_decorator
   def my_func():
       return "Hello"
   
   my_func()
   ```
   Expected: `unused_decorator` marked as unused

5. **Class method decorators**
   ```python
   def class_decorator(cls):
       return cls
   
   def method_decorator(func):
       return func
   
   @class_decorator  # Should be marked as used
   class MyClass:
       @method_decorator  # Should be marked as used
       def method(self):
           return "Method"
   
   obj = MyClass()
   obj.method()
   ```
   Expected: No unused warnings for decorators

6. **Decorator from imported module (if supported)**
   ```python
   import functools
   
   @functools.wraps  # Should track wraps as used
   def decorated(func):
       return func
   ```

7. **Nested decorator definitions**
   ```python
   def outer():
       def inner_decorator(func):
           return func
       return inner_decorator
   
   @outer()  # Should mark outer as used
   def func():
       pass
   
   func()
   ```

**Test Count**: 7-10 tests

#### Step 5: Update Integration Tests
**File**: `crates/silk-semantic/tests/test_integration.rs`

- Remove or update `test_decorator_with_control_flow` to reflect new behavior
- Update comment explaining decorator tracking now works

**Completion Criteria**:
- [ ] Decorator expressions analyzed for usage
- [ ] Simple decorators (@name) tracked
- [ ] Parametrized decorators (@name(args)) tracked
- [ ] Attribute decorators (@module.name) tracked
- [ ] Multiple decorators on same function tracked
- [ ] All new tests pass
- [ ] Integration tests updated
- [ ] Update CHANGELOG.md
- [ ] Update TODO.md

**Estimated Effort**: Small-Medium (1-2 implementation sessions)

---

## Type System Limitations

### 3. Generic Type Constraints Not Enforced

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

---

### 4. Type Narrowing Not Supported

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

---

## Code Generation Limitations

### 5. No Code Generation Yet

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

---

## Testing Limitations

### 6. Some Complex Control Flow Patterns Not Tested

**Status**: ✅ **RESOLVED** (December 12, 2025)

**Previous Issue**: While control flow analysis was comprehensive, many edge cases and complex exception handling patterns were not explicitly tested, leaving uncertainty about analyzer correctness.

**Solution**: Created comprehensive test suite `test_complex_exception_patterns.rs` with 15 tests covering previously untested complex patterns.

**Patterns Now Tested**:
- Break and continue in finally blocks
- Return in except handlers with else clauses
- Exception variable scope and usage
- Multiple exception types in single handler: `except (ValueError, TypeError, KeyError):`
- Deeply nested try blocks (3+ levels of nesting)
- Try/except blocks within conditionals
- Bare raise statements in except handlers
- Nested finally blocks
- Complete try/except/else/finally combinations
- Return statement precedence across try/except/finally
- Exception handling within exception handlers (nested try)
- Try/except in while loops with break/continue
- Except handler ordering (broad exception types before specific)

**Testing Coverage**:
- New file: `test_complex_exception_patterns.rs` (15 tests)
- Existing: `test_try_except_reachability.rs` (15 tests)
- Combined: 30 comprehensive exception control flow tests

**Discovered Current Limitations** (documented, not bugs):
- Bare `raise` statements not currently tracked as diverging control flow (code after is reachable)
- Try block with return + except without return: code after is marked reachable
- Try/except in conditionals: all-paths-return not fully tracked across try blocks

These limitations represent areas for future enhancement but do not indicate bugs in current implementation.

**Impact**: Significantly increased confidence in exception handling analysis through comprehensive edge case testing.

---

## Resolved Limitations

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
- All statement types with inline comments
- Edge cases (end of file, empty comments, etc.)
- Hash symbols in strings vs. comments
- Line number preservation

**See**: CHANGELOG.md for full details

---

### ✅ Global/Nonlocal Statement Support (Resolved December 12, 2025)

**Previous Issue**: Variables declared with `global` or `nonlocal` statements were not marked as initialized, causing false "uninitialized variable" errors in control flow analysis.

**Solution**: Implemented handling in `control_flow.rs` to mark variables as initialized when they appear in `global` or `nonlocal` statements.

**Impact**: Eliminates false positives for legitimate closure and global variable patterns common in Python code.

**Implementation**:
- Modified `analyze_statement()` in `ControlFlowAnalyzer`
- Added cases for `StatementKind::Global` and `StatementKind::Nonlocal`
- Mark all listed variables as initialized

**Tests Added**: 18 comprehensive tests in `test_global_nonlocal.rs`
- Basic global/nonlocal functionality
- Multiple variables in single statement
- Nested function contexts
- Combined usage patterns
- Edge cases

**See**: CHANGELOG.md for full details

---

## Summary

| # | Limitation | Priority | Difficulty | Status |
|---|------------|----------|------------|--------|
| 1 | Lambda parameter defaults | Medium | Easy | Parser limitation |
| 2 | Complex comprehension type inference | Low | Medium | Partial support |
| 3 | Method call tracking | Medium | Medium | Documented workaround |
| 4 | Decorator usage tracking | Medium | Easy | Documented workaround |
| 5 | Generic type constraints | Low | Hard | Future enhancement |
| 6 | Type narrowing | Low | Hard | Future enhancement |
| 7 | Code generation | High | Very Hard | Phase 7+ |
| 8 | Test coverage gaps | Medium | Easy | Ongoing |

---

## Contributing

When adding a new limitation to this document:

1. **Describe the limitation clearly** with examples
2. **Explain the impact** on users
3. **Provide a workaround** if available
4. **Create a step-by-step implementation plan** with:
   - File locations
   - Code changes needed
   - Test cases required
   - Completion criteria
5. **Estimate the effort** (Easy/Medium/Hard)
6. **Update the summary table**

When a limitation is resolved:
- Move it to CHANGELOG.md under the version where it was fixed
- Reference the issue/PR that fixed it
- Keep the documentation for historical reference

---

*Last Updated: December 12, 2025*
