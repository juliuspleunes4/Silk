# Function Call Type Inference Implementation Steps

**Goal**: Implement type inference for function calls based on function return types

**Branch**: `feat/function-call-type-inference`

**Status**: 🚀 IN PROGRESS

---

## Overview

Implement type inference so that when you call a function, the result type is inferred from the function's return type annotation:

```python
def add(x: int, y: int) -> int:
    return x + y

result = add(5, 10)  # Should infer result is int
```

**What exists:**
- ✅ AST: `FunctionDef` has `returns: Option<Type>` field
- ✅ Symbol Table: Can store function symbols
- ✅ Type System: `resolve_type_annotation()` method ready
- ❌ Function symbols don't store return type yet
- ❌ `infer_type()` doesn't handle `Call` expressions yet

**What we need to build:**
1. Store function return types in symbol table
2. Add `infer_call_type()` method
3. Look up function symbol and get return type
4. Handle built-in functions
5. Comprehensive tests

---

## Step-by-Step Implementation

### ✅ Step 1: Understand Current Function Handling (ANALYSIS ONLY)

**Task**: Understand how functions are currently stored in symbol table

**Files to read:**
- `crates/silk-semantic/src/analyzer.rs` - Find `StatementKind::FunctionDef` handling
- `crates/silk-semantic/src/symbol_table.rs` - See how `Symbol` is stored
- `crates/silk-semantic/src/types.rs` - Check if Type needs extension

**Questions to answer:**
1. Where are function symbols created?
2. What type do they currently get? (Unknown?)
3. Do we need a new Type variant for functions?
4. How is the return type annotation accessed?

**Action**: Read code, no changes yet

---

### ⏳ Step 2: Extend Type System for Functions (SMALL CHANGE)

**File**: `crates/silk-semantic/src/types.rs`

**Decision needed**: How to represent function types?

**Option A**: Add `Function { return_type: Box<Type> }` variant
- Stores only return type
- Simple, matches current need
- Can extend later with parameter types

**Option B**: Add `Function { params: Vec<Type>, return_type: Box<Type> }` variant
- Stores full signature
- More future-proof
- More complex upfront

**Recommendation**: Start with Option A

**Changes:**
```rust
pub enum Type {
    // ... existing variants ...
    
    /// Function type with return type
    Function {
        return_type: Box<Type>,
    },
}
```

**Also update:**
- `is_compatible_with()` - Functions are compatible if return types are
- `as_str()` - Display as "function -> ReturnType"
- `from_str()` - Not needed (functions aren't parsed from strings)

**Estimated lines**: ~20 lines

---

### ⏳ Step 3: Store Function Return Types in Symbol Table (SMALL CHANGE)

**File**: `crates/silk-semantic/src/analyzer.rs`

**Task**: When analyzing `FunctionDef`, resolve return type annotation and store it

**Current code** (around line 166):
```rust
StatementKind::FunctionDef { params, body, decorator_list, .. } => {
    // Current implementation
}
```

**New logic needed:**
1. Check if `returns` field exists
2. If yes, call `resolve_type_annotation(returns)`
3. Create function symbol with `Type::Function { return_type }`
4. If no return type annotation, use `Type::Unknown`

**Pseudo-code:**
```rust
StatementKind::FunctionDef { name, returns, .. } => {
    let return_type = returns
        .as_ref()
        .map(|rt| self.resolve_type_annotation(rt))
        .unwrap_or(Type::Unknown);
    
    let func_type = Type::Function {
        return_type: Box::new(return_type),
    };
    
    let symbol = Symbol::with_type(
        name.clone(),
        SymbolKind::Function,
        stmt.span.clone(),
        func_type,
    );
    
    // Define symbol...
}
```

**Estimated lines**: ~10 lines modified

---

### ⏳ Step 4: Create Basic Tests for Function Symbol Storage (TEST FIRST)

**File**: `crates/silk-semantic/tests/test_function_types.rs` (NEW FILE)

**Tests to write:**
1. `test_function_with_return_type_creates_symbol()` - Basic function with `-> int`
2. `test_function_without_return_type_unknown()` - Function with no annotation
3. `test_function_symbol_has_function_type()` - Verify Type::Function is stored
4. `test_function_return_type_resolution()` - Check return type is correct

**Expected results**: Tests should PASS after Step 3 implementation

---

### ⏳ Step 5: Implement infer_call_type() Method (CORE LOGIC)

**File**: `crates/silk-semantic/src/analyzer.rs`

**Task**: Add method to infer type from function calls

**Location**: Add after `infer_unary_op_type()` method

**Implementation:**
```rust
/// Infer the type of a function call expression.
fn infer_call_type(&self, func: &Expression, args: &[Expression], keywords: &[CallKeyword]) -> Type {
    // Get the function expression type
    match &func.kind {
        ExpressionKind::Identifier(func_name) => {
            // Look up function in symbol table
            if let Some(symbol) = self.symbol_table.resolve_symbol(func_name) {
                match &symbol.ty {
                    Type::Function { return_type } => {
                        // Return the function's return type
                        return *return_type.clone();
                    }
                    _ => {
                        // Not a function, return Unknown
                        return Type::Unknown;
                    }
                }
            }
            
            // Check if it's a built-in function
            match func_name.as_str() {
                "len" => return Type::Int,
                "str" => return Type::Str,
                "int" => return Type::Int,
                "float" => return Type::Float,
                "bool" => return Type::Bool,
                "print" => return Type::None,
                "range" => return Type::Unknown, // TODO: Implement range type
                _ => return Type::Unknown,
            }
        }
        
        // Method calls, attribute calls, etc.
        _ => Type::Unknown,
    }
}
```

**Estimated lines**: ~40 lines

---

### ⏳ Step 6: Wire infer_call_type() into infer_type() (SMALL CHANGE)

**File**: `crates/silk-semantic/src/analyzer.rs`

**Task**: Add `Call` case to `infer_type()` method

**Find the `infer_type()` method** and add:
```rust
fn infer_type(&self, expr: &Expression) -> Type {
    match &expr.kind {
        // ... existing cases ...
        
        ExpressionKind::Call { func, args, keywords } => {
            self.infer_call_type(func, args, keywords)
        }
        
        // ... rest of cases ...
    }
}
```

**Estimated lines**: ~5 lines

---

### ⏳ Step 7: Create Basic Call Type Inference Tests

**File**: `crates/silk-semantic/tests/test_call_type_inference.rs` (NEW FILE)

**Tests to write:**
1. `test_call_to_function_with_int_return()` - Call function returning int
2. `test_call_to_function_with_str_return()` - Call function returning str
3. `test_call_to_function_without_return_type()` - Should return Unknown
4. `test_assignment_from_function_call()` - `x = foo()` should have foo's return type
5. `test_builtin_len_returns_int()` - `len([1,2,3])` returns int
6. `test_builtin_str_returns_str()` - `str(123)` returns str
7. `test_builtin_print_returns_none()` - `print("hi")` returns None

**Run**: Should pass after Steps 5-6 implementation

---

### ⏳ Step 8: Add Advanced Call Type Inference Tests

**File**: `crates/silk-semantic/tests/test_call_type_inference.rs`

**Additional tests:**
1. `test_nested_function_calls()` - `int(str(42))`
2. `test_function_call_in_expression()` - `result = foo() + bar()`
3. `test_function_call_as_argument()` - `print(get_message())`
4. `test_multiple_calls_to_same_function()` - Verify consistency
5. `test_recursive_function_call()` - Function calls itself
6. `test_method_call_returns_unknown()` - `obj.method()` not yet supported

---

### ⏳ Step 9: Handle Edge Cases

**File**: `crates/silk-semantic/src/analyzer.rs`

**Edge cases to handle:**

1. **Undefined function calls**:
   - What if function doesn't exist? → Already returns Unknown (symbol lookup fails)
   
2. **Calling non-functions**:
   - `x = 5; x()` → Already returns Unknown (symbol.ty is not Function)

3. **Method calls**:
   - `obj.method()` → Return Unknown for now (future: attribute type system)

4. **Lambda calls**:
   - `(lambda x: x + 1)(5)` → Return Unknown for now (future: lambda type inference)

**Action**: Document these as TODO comments where handled

---

### ⏳ Step 10: Add Built-in Function Coverage

**File**: `crates/silk-semantic/src/analyzer.rs`

**Extend built-in function list in `infer_call_type()`:**

Common Python built-ins:
- `len(...)` → Int
- `str(...)` → Str  
- `int(...)` → Int
- `float(...)` → Float
- `bool(...)` → Bool
- `type(...)` → Unknown (future: type objects)
- `print(...)` → None
- `input(...)` → Str
- `abs(...)` → preserve numeric type (Int or Float)
- `min(...), max(...)` → preserve argument type
- `sum(...)` → preserve numeric type
- `list(...), dict(...), set(...), tuple(...)` → Unknown (future: collection types)

**Action**: Add these to the match statement

**Tests**: Add test file `test_builtin_functions.rs` with ~10 tests

---

### ⏳ Step 11: Update Documentation

**Files to update:**

1. **README.md**:
   - Update test count (will increase by ~20+ tests)
   - Mention function call type inference in features

2. **CHANGELOG.md**:
   - Add new section: "🔧 Function Call Type Inference - December 11, 2025"
   - Document Type::Function variant
   - Document infer_call_type() method
   - List built-in functions supported
   - Show example usage
   - Note limitations (methods, lambdas)

3. **TODO.md**:
   - Update Type Inference from 40% → 60%
   - Mark "Function call type inference" as ✅ COMPLETED
   - Update Phase 2 progress from 85% → 90%

---

### ⏳ Step 12: Final Validation

**Tasks:**
1. Run full test suite: `cargo test --workspace`
2. Verify all new tests pass
3. Verify no existing tests broke
4. Check compilation: `cargo build --workspace`
5. Run clippy: `cargo clippy --workspace`
6. Review all changed files
7. Ensure documentation is synchronized

---

### ⏳ Step 13: Commit and Push

**Commands:**
```bash
git add .
git commit -m "feat: implement function call type inference with built-in function support"
git push origin feat/function-call-type-inference
```

**Commit message should include:**
- What was implemented
- Test count changes
- Files modified
- Known limitations

---

## Success Criteria

✅ `Type::Function` variant added to type system
✅ Function return types stored in symbol table
✅ `infer_call_type()` method implemented
✅ Built-in functions return correct types
✅ 20+ new tests pass (function types + call inference + builtins)
✅ All existing tests still pass
✅ Documentation updated (README, CHANGELOG, TODO)
✅ No compiler warnings

---

## Estimated Time

- Steps 1-4 (Type system + storage): 1-2 hours
- Steps 5-7 (Core inference): 1-2 hours
- Steps 8-10 (Advanced + built-ins): 1-2 hours
- Steps 11-13 (Docs + final): 30 minutes

**Total**: 3.5-6.5 hours

---

## Current Status Tracking

- [ ] Step 1: Analysis (understand current code)
- [ ] Step 2: Extend Type system for functions
- [ ] Step 3: Store function return types
- [ ] Step 4: Create basic function symbol tests
- [ ] Step 5: Implement infer_call_type()
- [ ] Step 6: Wire into infer_type()
- [ ] Step 7: Create basic call inference tests
- [ ] Step 8: Add advanced call tests
- [ ] Step 9: Handle edge cases
- [ ] Step 10: Add built-in function coverage
- [ ] Step 11: Update documentation
- [ ] Step 12: Final validation
- [ ] Step 13: Commit and push

**Next Step**: Step 1 (Analysis)

---

## Notes

- This builds on AnnAssign work (uses `resolve_type_annotation()`)
- Lays foundation for parameter type checking (future)
- Does NOT implement parameter validation yet (that's type checking, not inference)
- Methods and lambda calls return Unknown for now
