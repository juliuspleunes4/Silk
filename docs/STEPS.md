# Type Checking Implementation - Step-by-Step Plan

**Feature**: Type Checking for Silk Semantic Analyzer
**Branch**: `feat/type-checking`
**Goal**: Validate type compatibility for assignments, operations, function calls, and returns

## Overview

Type checking builds on the existing type inference system to validate that operations are type-safe. This includes:
- Assignment type checking (annotated variables must receive compatible values)
- Function argument type checking (calls must match parameter types)
- Binary operation validation (operands must be compatible)
- Return type validation (returns must match declared type)
- Collection operation validation (subscript/attribute access on correct types)

## Implementation Strategy

Break this into 7 phases with incremental testing:

### Phase 1: Error Infrastructure (Steps 1-3)
Add new error types for type mismatches

### Phase 2: Assignment Type Checking (Steps 4-7)
Validate annotated assignments receive compatible values

### Phase 3: Function Call Type Checking (Steps 8-11)
Validate function arguments match parameter types

### Phase 4: Return Type Checking (Steps 12-14)
Validate return values match declared return types

### Phase 5: Binary Operation Validation (Steps 15-17)
Validate binary operations have compatible operands

### Phase 6: Collection Operations (Steps 18-20)
Validate subscript and attribute access

### Phase 7: Integration & Documentation (Steps 21-25)
Advanced tests, full validation, documentation updates

---

## Detailed Steps

### **Phase 1: Error Infrastructure**

#### Step 1: Add Type Error Variants to SemanticError
- **File**: `crates/silk-semantic/src/error.rs`
- **Action**: Add new error variants for type mismatches
- **Add**:
  - `TypeMismatch` - general type incompatibility
  - `ArgumentTypeMismatch` - function call argument type error
  - `ReturnTypeMismatch` - return value type error
  - `InvalidOperation` - unsupported operation on types
- **Test**: Ensure errors compile and display correctly

#### Step 2: Create Test File for Type Checking Errors
- **File**: `crates/silk-semantic/tests/test_type_errors.rs`
- **Action**: Create test file structure
- **Tests**: 5-10 tests for error creation and display
- **Verify**: All tests pass

#### Step 3: Add Type Compatibility Helper Methods
- **File**: `crates/silk-semantic/src/types.rs`
- **Action**: Add utility methods
- **Add**:
  - `check_compatible(&self, &Type) -> bool` (wrapper for is_compatible_with)
  - `requires_exact_match(&self) -> bool` (for strict type checking)
- **Test**: 5 unit tests in types.rs

---

### **Phase 2: Assignment Type Checking**

#### Step 4: Analyze Current Annotated Assignment Handling
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Review existing `StatementKind::AnnAssign` handling
- **Document**: Current behavior (defines symbol with type from annotation)
- **Plan**: Where to add type checking logic

#### Step 5: Implement Assignment Type Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add type checking in `analyze_statement` for AnnAssign
- **Logic**:
  - If value is present, infer its type
  - Check if inferred type is compatible with annotation type
  - Add error if incompatible
- **Method**: Add `check_assignment_type()` helper (~30 lines)

#### Step 6: Add Assignment Type Checking Tests
- **File**: `crates/silk-semantic/tests/test_assignment_type_checking.rs`
- **Action**: Create comprehensive test file
- **Tests** (15 tests):
  - Valid: `x: int = 42` ✓
  - Invalid: `x: int = "hello"` ✗
  - Valid: `x: str = "hello"` ✓
  - Invalid: `x: str = 3.14` ✗
  - Valid: `x: list[int] = [1, 2, 3]` ✓
  - Invalid: `x: list[int] = [1, "a", 3]` ✗
  - Valid: `x: dict[str, int] = {"a": 1}` ✓
  - Invalid: `x: dict[str, int] = {"a": "b"}` ✗
  - Unknown assignments (should pass - no type annotation)
  - Multiple assignments with errors
  - Nested collection types
  - Assignment to Unknown variable (should warn/allow)

#### Step 7: Verify Assignment Type Checking
- **Action**: Run all tests
- **Verify**: New tests pass, existing 677 tests still pass
- **Debug**: Fix any issues

---

### **Phase 3: Function Call Type Checking**

#### Step 8: Analyze Function Parameter Type Storage
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Review how function parameter types are stored
- **Current**: Parameters defined in pre-pass or function body
- **Plan**: Ensure parameter types are accessible during call validation

#### Step 9: Implement Parameter Type Storage in Function Symbols
- **File**: `crates/silk-semantic/src/symbol_table.rs`
- **Action**: Extend Function symbol to store parameter types
- **Add**: `param_types: Vec<(String, Type)>` to function symbols
- **Update**: Pre-pass to store parameter types from annotations

#### Step 10: Implement Function Call Argument Type Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add type checking for function calls
- **Logic**:
  - When analyzing Call expression, check if callee is a function
  - Compare argument types with parameter types
  - Check argument count matches
  - Add error for mismatches
- **Method**: Add `check_function_call_types()` helper (~50 lines)

#### Step 11: Add Function Call Type Checking Tests
- **File**: `crates/silk-semantic/tests/test_function_call_type_checking.rs`
- **Action**: Create comprehensive test file
- **Tests** (15 tests):
  - Valid: `def f(x: int): ...` called with `f(42)` ✓
  - Invalid: `def f(x: int): ...` called with `f("hello")` ✗
  - Valid: `def f(x: int, y: str): ...` called with `f(1, "a")` ✓
  - Invalid: `def f(x: int, y: str): ...` called with `f("a", 1)` ✗
  - Too many arguments
  - Too few arguments
  - Mixed valid and invalid arguments
  - Function with no type annotations (should pass)
  - Nested function calls
  - Built-in function calls (type checking for built-ins)
  - Call with Unknown arguments (should pass)
  - Multiple errors in single call

---

### **Phase 4: Return Type Checking**

#### Step 12: Implement Return Statement Type Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add type checking for return statements
- **Logic**:
  - Track current function's return type
  - When analyzing Return statement, infer return value type
  - Check if compatible with declared return type
  - Add error if incompatible
- **Method**: Add `check_return_type()` helper (~30 lines)
- **Context**: Add `current_function_return_type: Option<Type>` to analyzer

#### Step 13: Add Return Type Checking Tests
- **File**: `crates/silk-semantic/tests/test_return_type_checking.rs`
- **Action**: Create comprehensive test file
- **Tests** (12 tests):
  - Valid: `def f() -> int: return 42` ✓
  - Invalid: `def f() -> int: return "hello"` ✗
  - Valid: `def f() -> str: return "hello"` ✓
  - Invalid: `def f() -> str: return 3.14` ✗
  - Valid: `def f() -> list[int]: return [1, 2, 3]` ✓
  - Invalid: `def f() -> list[int]: return [1, "a"]` ✗
  - Function with no return type (should pass)
  - Empty return in function with return type
  - Multiple returns with errors
  - Return Unknown (should pass)
  - Nested function returns

#### Step 14: Verify Return Type Checking
- **Action**: Run all tests
- **Verify**: New tests pass, all existing tests still pass
- **Debug**: Fix any issues

---

### **Phase 5: Binary Operation Validation**

#### Step 15: Analyze Current Binary Operation Type Inference
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Review `infer_binary_type()` method
- **Current**: Infers result type of binary operations
- **Plan**: Add validation that operands are compatible

#### Step 16: Implement Binary Operation Type Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add validation in `infer_binary_type()`
- **Logic**:
  - Before inferring result type, check operand compatibility
  - For arithmetic: both should be numeric (int/float)
  - For string concat: both should be strings
  - For comparisons: operands should be comparable
  - Add error for invalid operations
- **Method**: Modify `infer_binary_type()` to validate (~20 lines added)

#### Step 17: Add Binary Operation Validation Tests
- **File**: `crates/silk-semantic/tests/test_binary_operation_validation.rs`
- **Action**: Create comprehensive test file
- **Tests** (15 tests):
  - Valid: `1 + 2` (int + int) ✓
  - Valid: `1 + 2.5` (int + float) ✓
  - Invalid: `1 + "hello"` (int + str) ✗
  - Valid: `"hello" + "world"` (str + str) ✓
  - Invalid: `"hello" * 3.14` (str * float) ✗
  - Valid: `1 < 2` (int < int) ✓
  - Valid: `1.5 > 2` (float > int) ✓
  - Invalid: `1 < "hello"` (int < str) ✗
  - Unknown operands (should pass)
  - Multiple operations with errors
  - Nested operations

---

### **Phase 6: Collection Operations**

#### Step 18: Implement Subscript Type Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add type checking for subscript operations
- **Logic**:
  - Check if subscripted value is a collection (list/dict/tuple/str)
  - For lists/tuples: index should be int
  - For dicts: index should match key type
  - Add error for invalid subscripts
- **Method**: Add `check_subscript_type()` helper (~40 lines)

#### Step 19: Implement Attribute Access Validation
- **File**: `crates/silk-semantic/src/analyzer.rs`
- **Action**: Add basic attribute access validation
- **Logic**:
  - For now, just track that attribute access is on Unknown or known types
  - Future: Full class/module attribute validation
  - This step is mostly preparation for future work
- **Method**: Add `check_attribute_access()` helper (~20 lines)

#### Step 20: Add Collection Operation Tests
- **File**: `crates/silk-semantic/tests/test_collection_operations.rs`
- **Action**: Create comprehensive test file
- **Tests** (12 tests):
  - Valid: `[1, 2, 3][0]` (list[int] subscript) ✓
  - Invalid: `[1, 2, 3]["a"]` (list[int] with str index) ✗
  - Valid: `{"a": 1}["a"]` (dict[str, int] subscript) ✓
  - Invalid: `{"a": 1}[0]` (dict[str, int] with int index) ✗
  - Valid: `(1, 2, 3)[1]` (tuple subscript) ✓
  - Invalid: `(1, 2, 3)["x"]` (tuple with str index) ✗
  - Subscript on Unknown (should pass)
  - Nested subscripts
  - Multiple errors

---

### **Phase 7: Integration & Documentation**

#### Step 21: Create Advanced Type Checking Tests
- **File**: `crates/silk-semantic/tests/test_advanced_type_checking.rs`
- **Action**: Test complex scenarios
- **Tests** (10 tests):
  - Annotated assignment with function call result
  - Function returning collection used in for loop
  - Nested function calls with type checking
  - Assignment chain with type checking
  - Mixed valid and invalid operations
  - Type checking with forward references
  - Type checking in class methods
  - Comprehensions with type checking
  - Lambda return type checking
  - Complex nested scenarios

#### Step 22: Run Full Test Suite
- **Action**: `cargo test --workspace`
- **Verify**: All tests pass (expect ~750+ tests)
- **Count**: Get exact test count
- **Debug**: Fix any failures

#### Step 23: Run Clippy and Format
- **Action**: `cargo clippy --workspace` and `cargo fmt --workspace`
- **Fix**: Any warnings
- **Verify**: Clean build

#### Step 24: Update Documentation
- **Files**: README.md, CHANGELOG.md, TODO.md
- **README**: Update test count badge and breakdown
- **CHANGELOG**: Add comprehensive Type Checking section with examples
- **TODO**: Update progress (Phase 2: 95% → 98%, Type Checking 0% → 100%)

#### Step 25: Final Verification and Cleanup
- **Action**: Delete STEPS.md
- **Verify**: All files formatted, no TODOs in code
- **Commit**: Comprehensive commit message
- **Push**: Push to feat/type-checking branch

---

## Success Criteria

- ✅ All assignment type checking works
- ✅ All function call type checking works
- ✅ All return type checking works
- ✅ Binary operation validation works
- ✅ Collection operation validation works
- ✅ 70-80 new tests added (estimated)
- ✅ All tests passing (~750+ total)
- ✅ Clean clippy output
- ✅ Documentation updated

## Progress Tracking

- [x] Phase 1: Error Infrastructure (Steps 1-3) ✅ COMPLETE
  - [x] Step 1: Add Type Error Variants ✅ (10 tests passing)
  - [x] Step 2: Create Test File ✅ (combined with Step 1)
  - [x] Step 3: Add Type Compatibility Helpers ✅ (23 tests passing)
- [ ] Phase 2: Assignment Type Checking (Steps 4-7)
- [ ] Phase 3: Function Call Type Checking (Steps 8-11)
- [ ] Phase 4: Return Type Checking (Steps 12-14)
- [ ] Phase 5: Binary Operation Validation (Steps 15-17)
- [ ] Phase 6: Collection Operations (Steps 18-20)
- [ ] Phase 7: Integration & Documentation (Steps 21-25)
