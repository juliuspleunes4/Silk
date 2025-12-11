# Collection Type Inference Implementation Plan

## Goal
Implement type inference for Python collection literals (list, dict, set, tuple) to enable proper type tracking and validation.

## Overview
Collections are fundamental Python types. We need to:
1. Extend the Type system to represent generic collections
2. Implement inference logic to analyze collection elements
3. Handle edge cases (empty, heterogeneous, nested)
4. Write comprehensive tests

---

## Steps

### Step 1: Analyze Current State
**Goal**: Understand how collections are currently handled
- [ ] Check `ExpressionKind` variants for List, Dict, Set, Tuple in AST
- [ ] Review current `infer_type()` handling (likely returns Unknown)
- [ ] Check existing tests for collection literals

### Step 2: Extend Type System - List
**Goal**: Add `Type::List` variant with element type
- [ ] Add `List(Box<Type>)` variant to `Type` enum in `types.rs`
- [ ] Update `Display` trait: `"list[ElementType]"`
- [ ] Update `is_compatible_with()` for list type compatibility
- [ ] Update `from_str()` if needed

### Step 3: Extend Type System - Dict
**Goal**: Add `Type::Dict` variant with key/value types
- [ ] Add `Dict { key_type: Box<Type>, value_type: Box<Type> }` variant
- [ ] Update `Display` trait: `"dict[KeyType, ValueType]"`
- [ ] Update `is_compatible_with()` for dict type compatibility

### Step 4: Extend Type System - Set
**Goal**: Add `Type::Set` variant with element type
- [ ] Add `Set(Box<Type>)` variant
- [ ] Update `Display` trait: `"set[ElementType]"`
- [ ] Update `is_compatible_with()` for set type compatibility

### Step 5: Extend Type System - Tuple
**Goal**: Add `Type::Tuple` variant with heterogeneous element types
- [ ] Add `Tuple(Vec<Type>)` variant (each element can have different type)
- [ ] Update `Display` trait: `"tuple[Type1, Type2, ...]"`
- [ ] Update `is_compatible_with()` for tuple type compatibility
- [ ] Special case: empty tuple `tuple[()]`

### Step 6: Create Type System Tests
**Goal**: Verify Type enum extensions work correctly
- [ ] Create `test_collection_types.rs` test file
- [ ] Test Type::List Display and compatibility
- [ ] Test Type::Dict Display and compatibility
- [ ] Test Type::Set Display and compatibility
- [ ] Test Type::Tuple Display and compatibility (including empty)
- [ ] Run tests, verify all pass

### Step 7: Implement List Inference Helper
**Goal**: Add method to infer list element types
- [ ] Add `infer_list_type()` method to analyzer
- [ ] Analyze all elements, collect their types
- [ ] Find common type (all same → that type, mixed → Unknown)
- [ ] Handle empty list → `list[Unknown]`
- [ ] Add TODO comments for future union type support
- [ ] Document edge cases

### Step 8: Wire List Inference into infer_type()
**Goal**: Connect list inference to main type inference pipeline
- [ ] Add `ExpressionKind::List` case in `infer_type()`
- [ ] Call `infer_list_type()` helper
- [ ] Return inferred `Type::List`

### Step 9: Create Basic List Inference Tests
**Goal**: Verify list type inference works
- [ ] Create `test_list_type_inference.rs` test file
- [ ] Test homogeneous int list: `[1, 2, 3]` → `list[int]`
- [ ] Test homogeneous str list: `["a", "b"]` → `list[str]`
- [ ] Test homogeneous float list: `[1.0, 2.0]` → `list[float]`
- [ ] Test empty list: `[]` → `list[Unknown]`
- [ ] Test heterogeneous list: `[1, "a"]` → `list[Unknown]`
- [ ] Test nested list: `[[1, 2], [3, 4]]` → `list[list[int]]`
- [ ] Test list with variables
- [ ] Run tests, verify all pass

### Step 10: Implement Dict Inference Helper
**Goal**: Add method to infer dict key/value types
- [ ] Add `infer_dict_type()` method to analyzer
- [ ] Analyze all keys, find common key type
- [ ] Analyze all values, find common value type
- [ ] Handle empty dict → `dict[Unknown, Unknown]`
- [ ] Handle heterogeneous → `dict[Unknown, Unknown]`
- [ ] Document edge cases

### Step 11: Wire Dict Inference into infer_type()
**Goal**: Connect dict inference to main pipeline
- [ ] Add `ExpressionKind::Dict` case in `infer_type()`
- [ ] Call `infer_dict_type()` helper
- [ ] Return inferred `Type::Dict`

### Step 12: Create Basic Dict Inference Tests
**Goal**: Verify dict type inference works
- [ ] Create `test_dict_type_inference.rs` test file
- [ ] Test homogeneous int→str: `{1: "a", 2: "b"}` → `dict[int, str]`
- [ ] Test homogeneous str→int: `{"a": 1, "b": 2}` → `dict[str, int]`
- [ ] Test empty dict: `{}` → `dict[Unknown, Unknown]`
- [ ] Test heterogeneous keys: `{1: "a", "b": 2}` → `dict[Unknown, Unknown]`
- [ ] Test heterogeneous values: `{"a": 1, "b": "c"}` → `dict[str, Unknown]`
- [ ] Test nested dict values
- [ ] Run tests, verify all pass

### Step 13: Implement Set Inference Helper
**Goal**: Add method to infer set element types
- [ ] Add `infer_set_type()` method to analyzer
- [ ] Analyze all elements, collect their types
- [ ] Find common type (similar to list)
- [ ] Handle empty set: note `{}` is empty dict, `set()` is call
- [ ] Document that empty set literals don't exist in Python syntax

### Step 14: Wire Set Inference into infer_type()
**Goal**: Connect set inference to main pipeline
- [ ] Add `ExpressionKind::Set` case in `infer_type()`
- [ ] Call `infer_set_type()` helper
- [ ] Return inferred `Type::Set`

### Step 15: Create Basic Set Inference Tests
**Goal**: Verify set type inference works
- [ ] Create `test_set_type_inference.rs` test file
- [ ] Test homogeneous int set: `{1, 2, 3}` → `set[int]`
- [ ] Test homogeneous str set: `{"a", "b"}` → `set[str]`
- [ ] Test heterogeneous set: `{1, "a"}` → `set[Unknown]`
- [ ] Test nested sets (if supported by parser)
- [ ] Note: empty set not testable (no syntax for it)
- [ ] Run tests, verify all pass

### Step 16: Implement Tuple Inference Helper
**Goal**: Add method to infer tuple element types
- [ ] Add `infer_tuple_type()` method to analyzer
- [ ] Analyze each element individually (tuples are heterogeneous!)
- [ ] Return `Type::Tuple(vec![type1, type2, ...])`
- [ ] Handle empty tuple: `Type::Tuple(vec![])`
- [ ] Handle single element tuple: `(x,)` → `tuple[TypeOfX]`

### Step 17: Wire Tuple Inference into infer_type()
**Goal**: Connect tuple inference to main pipeline
- [ ] Add `ExpressionKind::Tuple` case in `infer_type()`
- [ ] Call `infer_tuple_type()` helper
- [ ] Return inferred `Type::Tuple`

### Step 18: Create Basic Tuple Inference Tests
**Goal**: Verify tuple type inference works
- [ ] Create `test_tuple_type_inference.rs` test file
- [ ] Test homogeneous tuple: `(1, 2, 3)` → `tuple[int, int, int]`
- [ ] Test heterogeneous tuple: `(1, "a", 3.0)` → `tuple[int, str, float]`
- [ ] Test single element: `(42,)` → `tuple[int]`
- [ ] Test empty tuple: `()` → `tuple[]`
- [ ] Test nested tuple: `((1, 2), (3, 4))` → `tuple[tuple[int, int], tuple[int, int]]`
- [ ] Test tuple with variables
- [ ] Run tests, verify all pass

### Step 19: Add Advanced Collection Tests
**Goal**: Test complex scenarios and edge cases
- [ ] Test list comprehensions (should return `list[Unknown]` for now)
- [ ] Test dict comprehensions (should return `dict[Unknown, Unknown]`)
- [ ] Test set comprehensions (should return `set[Unknown]`)
- [ ] Test collections with function call results
- [ ] Test collections in assignments: `x: list[int] = [1, 2, 3]`
- [ ] Test nested heterogeneous collections
- [ ] Test collections with binary operations inside
- [ ] Run all tests, fix any issues

### Step 20: Update Type Compatibility Rules
**Goal**: Ensure collection types work with type checking
- [ ] Review `is_compatible_with()` for all collection types
- [ ] Ensure `list[int]` is compatible with `list[int]`
- [ ] Ensure `list[int]` is NOT compatible with `list[str]`
- [ ] Same for dict, set, tuple
- [ ] Add tests for type compatibility
- [ ] Consider: should `list[int]` be compatible with `list[Unknown]`?

### Step 21: Run Full Test Suite
**Goal**: Verify nothing broke
- [ ] Run `cargo test --workspace`
- [ ] Verify all existing tests still pass
- [ ] Count new tests added
- [ ] Run `cargo build --workspace`
- [ ] Run `cargo clippy --workspace`
- [ ] Fix any warnings in new code

### Step 22: Update Documentation - README
**Goal**: Document the new feature
- [ ] Update test count in README badge
- [ ] Update test breakdown (add collection inference tests)
- [ ] Update semantic analysis test count
- [ ] Verify math: total = lexer + parser + types + semantic

### Step 23: Update Documentation - CHANGELOG
**Goal**: Record what was implemented
- [ ] Add new section: "Collection Type Inference - December 11, 2025"
- [ ] Document Type system changes (4 new variants)
- [ ] Document analyzer changes (4 new helper methods)
- [ ] List example usage for each collection type
- [ ] List files modified
- [ ] List tests added with breakdown
- [ ] Document limitations (comprehensions return Unknown, etc.)

### Step 24: Update Documentation - TODO
**Goal**: Mark progress complete
- [ ] Update Type Inference: 60% → 80% complete
- [ ] Mark "Collection type inference" as ✅ COMPLETED
- [ ] Update Phase 2: 90% → 95% complete
- [ ] Verify Type Checking is still blocked (needs more work)
- [ ] Update progress summary date

### Step 25: Final Validation and Commit
**Goal**: Clean up and commit the feature
- [ ] Delete this STEPS.md file
- [ ] Review all changes one more time
- [ ] Stage all files: `git add .`
- [ ] Commit: `git commit -m "feat: implement collection type inference for list/dict/set/tuple"`
- [ ] Write detailed commit body with summary
- [ ] Push to branch: `git push -u origin feat/collection-type-inference`
- [ ] Verify branch is ready for PR/merge

---

## Expected Outcomes

**New Type Variants**: 4 (List, Dict, Set, Tuple)
**New Helper Methods**: 4 (infer_list_type, infer_dict_type, infer_set_type, infer_tuple_type)
**New Test Files**: 4-5 (collection types + inference tests for each collection)
**Estimated New Tests**: 40-60 tests
**Phase 2 Progress**: 90% → 95%
**Type Inference Progress**: 60% → 80%

## Notes

- **Homogeneous vs Heterogeneous**: Lists/dicts/sets should be homogeneous in typed Python. If mixed types, use Unknown.
- **Tuples are special**: Tuples can be heterogeneous (each position has its own type).
- **Empty collections**: Return `CollectionType[Unknown]` since we can't infer from nothing.
- **Comprehensions**: These are complex expressions; return Unknown for now (future work).
- **Union types**: Not implemented yet, so mixed types → Unknown.

## Future Enhancements (Not in this PR)
- Collection comprehension type inference
- Union types for heterogeneous collections
- Type narrowing based on annotations
- Generic type parameters validation
- Covariance/contravariance for collection types
