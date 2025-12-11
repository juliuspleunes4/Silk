# Control Flow Analysis Implementation Plan

**Feature**: Control Flow Analysis for Silk Compiler
**Branch**: `feat/control-flow-analysis`
**Start Date**: December 11, 2025
**Estimated Duration**: 2-3 weeks
**Test Goal**: Add ~100-150 comprehensive tests

---

## Overview

Control flow analysis adds compile-time validation to detect common programming errors:
- Unreachable code after returns/breaks/continues
- Uninitialized variable usage
- Missing return statements in functions
- Dead code that will never execute
- Infinite loop detection

This completes Phase 2 of semantic analysis before moving to code generation.

---

## Phase 1: Infrastructure Setup (Steps 1-4)

### Step 1: Create Control Flow Error Types
**File**: `crates/silk-semantic/src/error.rs`
**Estimated Tests**: 5-8

Add new error variants:
- `UnreachableCode` - Code after return/break/continue/raise
- `UninitializedVariable` - Variable used before initialization
- `MissingReturn` - Function missing return on some paths
- `InfiniteLoop` - Loop that never terminates (while True without break)
- `DeadCode` - Code that can never be executed

**Testing**:
- Create `tests/test_control_flow_errors.rs`
- Test each error type can be constructed
- Verify error messages and source locations

---

### Step 2: Create ControlFlowAnalyzer Structure
**File**: `crates/silk-semantic/src/control_flow.rs` (new file)
**Estimated Tests**: 0 (infrastructure)

Create basic analyzer structure:
```rust
pub struct ControlFlowAnalyzer {
    errors: Vec<SemanticError>,
    current_function_returns: bool,
    in_loop: bool,
}
```

Methods:
- `new()` - Constructor
- `analyze(&mut self, program: &Program)` - Entry point
- `errors(&self) -> &[SemanticError]` - Get collected errors

**Testing**: None yet (will be tested through usage)

---

### Step 3: Add Module to Crate
**File**: `crates/silk-semantic/src/lib.rs`
**Estimated Tests**: 0 (infrastructure)

Expose control flow module:
```rust
mod control_flow;
pub use control_flow::ControlFlowAnalyzer;
```

**Testing**: Ensure `cargo build` succeeds

---

### Step 4: Basic Statement Traversal
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 3-5

Implement basic AST traversal:
- `analyze_statement(&mut self, stmt: &Statement)`
- Handle all statement types with empty logic (preparation)
- Track whether we're inside a function/loop

**Testing**:
- Test analyzer runs without errors on empty program
- Test analyzer traverses all statement types
- Test basic function/loop context tracking

**Checkpoint**: 8-13 tests total
**Run**: `cargo test --package silk-semantic`

---

## Phase 2: Unreachable Code Detection (Steps 5-8)

### Step 5: Track Statement Reachability
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 8-10

Add reachability tracking:
- Track if current statement is reachable
- Mark code after `return` as unreachable
- Mark code after `break`/`continue` as unreachable
- Mark code after `raise` as unreachable

**Testing**:
- `test_unreachable_after_return`
- `test_unreachable_after_break`
- `test_unreachable_after_continue`
- `test_unreachable_after_raise`
- `test_reachable_in_if_branch` (both branches can have returns)
- `test_multiple_unreachable_statements`
- `test_nested_unreachable_code`
- `test_unreachable_in_try_block`

---

### Step 6: Handle Conditional Reachability
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 10-12

Handle branches (if/elif/else):
- Code is unreachable only if ALL branches return/break/etc
- Track reachability per branch
- Merge reachability after conditional

**Testing**:
- `test_reachable_after_if_no_else`
- `test_reachable_after_if_only_one_branch_returns`
- `test_unreachable_after_if_all_branches_return`
- `test_reachable_after_elif_chains`
- `test_unreachable_after_exhaustive_if_elif_else`
- `test_nested_conditionals_reachability`
- `test_conditional_in_loop`
- `test_early_return_in_nested_if`

---

### Step 7: Handle Loop Reachability
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 8-10

Handle loops (while/for):
- Code after loop is reachable (even with break inside)
- Code after `while True` without break is unreachable
- Handle loop `else` clause

**Testing**:
- `test_reachable_after_while_loop`
- `test_reachable_after_for_loop`
- `test_unreachable_after_infinite_loop_no_break`
- `test_reachable_after_infinite_loop_with_break`
- `test_loop_else_reachability`
- `test_nested_loops_reachability`
- `test_break_in_outer_loop`
- `test_continue_in_inner_loop`

---

### Step 8: Handle Try/Except Reachability
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 6-8

Handle exception handling:
- Code after try block is reachable (exceptions can occur)
- Track reachability through except handlers
- Handle finally clause (always executes)

**Testing**:
- `test_reachable_after_try_except`
- `test_unreachable_in_try_after_return`
- `test_reachable_in_except_handler`
- `test_finally_always_executes`
- `test_return_in_except_handler`
- `test_nested_try_except`

**Checkpoint**: 40-53 tests total (32-40 new)
**Run**: `cargo test --package silk-semantic`

---

## Phase 3: Variable Initialization Tracking (Steps 9-11)

### Step 9: Track Variable Definitions
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 8-10

Add variable initialization tracking:
- Track which variables are initialized in current scope
- Use HashSet to store initialized variable names
- Update on assignment statements

**Testing**:
- `test_variable_initialized_before_use`
- `test_uninitialized_variable_error`
- `test_function_parameter_always_initialized`
- `test_loop_variable_initialized`
- `test_multiple_assignments`
- `test_initialization_in_if_branch`
- `test_walrus_operator_initialization`
- `test_for_loop_target_initialization`

---

### Step 10: Handle Conditional Initialization
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 10-12

Track initialization through branches:
- Variable is initialized if initialized in ALL branches
- Merge initialization state after if/elif/else
- Handle initialization in loops

**Testing**:
- `test_uninitialized_from_conditional_branch`
- `test_initialized_in_all_branches`
- `test_initialized_in_if_not_else`
- `test_initialized_before_if_used_after`
- `test_conditional_initialization_in_loop`
- `test_nested_conditional_initialization`
- `test_initialization_in_try_except`
- `test_initialization_in_one_except_handler`
- `test_initialization_in_all_except_handlers`
- `test_elif_chain_initialization`

---

### Step 11: Function Parameters and Defaults
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 6-8

Handle function-specific initialization:
- Function parameters are always initialized
- Default parameter expressions checked separately
- Handle *args and **kwargs

**Testing**:
- `test_parameter_initialized_on_entry`
- `test_args_kwargs_initialized`
- `test_default_parameter_expression_checked`
- `test_parameter_shadows_outer_scope`
- `test_nested_function_parameter_scope`
- `test_lambda_parameter_initialization`

**Checkpoint**: 64-83 tests total (24-30 new)
**Run**: `cargo test --package silk-semantic`

---

## Phase 4: Return Path Validation (Steps 12-14)

### Step 12: Track Return Paths in Functions
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 10-12

Track whether function returns on all paths:
- Track if current path has returned
- Merge return state from branches
- Detect missing return at function end

**Testing**:
- `test_function_returns_on_all_paths`
- `test_missing_return_error`
- `test_function_with_no_return_type_ok`
- `test_return_in_if_else_all_branches`
- `test_missing_return_in_one_branch`
- `test_return_after_loop`
- `test_return_in_nested_function`
- `test_generator_function_no_return_needed`
- `test_implicit_none_return`
- `test_early_return_ok`

---

### Step 13: Handle Complex Return Patterns
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 8-10

Handle edge cases:
- Return in try/except/finally
- Return in loop (doesn't guarantee return)
- Return after raise (unreachable)
- Multiple return statements

**Testing**:
- `test_return_in_try_block`
- `test_return_in_except_handler`
- `test_return_in_finally_overrides`
- `test_return_in_loop_not_sufficient`
- `test_return_after_infinite_loop`
- `test_multiple_return_points`
- `test_conditional_return_with_raise`
- `test_nested_try_return`

---

### Step 14: Validate Return Types Match
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 6-8

Ensure return consistency:
- Functions with return type must return
- Functions without return type can omit return (implicit None)
- Detect inconsistent return patterns

**Testing**:
- `test_typed_function_must_return`
- `test_untyped_function_optional_return`
- `test_void_function_explicit_return_none`
- `test_return_type_annotation_enforced`
- `test_missing_return_with_type_hint`
- `test_all_paths_return_with_type_hint`

**Checkpoint**: 88-113 tests total (24-30 new)
**Run**: `cargo test --package silk-semantic`

---

## Phase 5: Dead Code Detection (Steps 15-17)

### Step 15: Detect Unused Variables
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 8-10

Track variable usage:
- Mark variables when assigned
- Mark variables when used
- Report unused variables (optional warning)

**Testing**:
- `test_unused_variable_warning`
- `test_used_variable_no_warning`
- `test_unused_function_parameter`
- `test_unused_in_nested_scope`
- `test_unused_loop_variable`
- `test_underscore_prefix_no_warning` (Python convention)
- `test_unused_in_comprehension`
- `test_unused_walrus_variable`

---

### Step 16: Detect Unused Functions
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 6-8

Track function definitions and calls:
- Mark functions when defined
- Mark functions when called
- Report unused functions (not called anywhere)

**Testing**:
- `test_unused_function_warning`
- `test_called_function_no_warning`
- `test_recursive_function_used`
- `test_mutually_recursive_functions`
- `test_unused_nested_function`
- `test_main_function_always_considered_used`
- `test_decorated_function_considered_used`

---

### Step 17: Optimize Dead Code Reporting
**File**: `crates/silk-semantic/src/control_flow.rs`
**Estimated Tests**: 6-8

Refine dead code detection:
- Don't report unreachable code in unreachable functions
- Distinguish errors vs warnings
- Add configuration for strictness level

**Testing**:
- `test_unreachable_in_unused_function_single_warning`
- `test_error_vs_warning_classification`
- `test_dead_code_after_return_error`
- `test_unused_import_warning`
- `test_suppress_warnings_in_dead_code`
- `test_cascading_unreachability`

**Checkpoint**: 108-139 tests total (20-26 new)
**Run**: `cargo test --package silk-semantic`

---

## Phase 6: Integration & Documentation (Steps 18-20)

### Step 18: Integrate with SemanticAnalyzer
**File**: `crates/silk-semantic/src/analyzer.rs`
**Estimated Tests**: 8-10

Add control flow analysis to main analyzer:
- Call `ControlFlowAnalyzer` after type checking
- Merge errors from both analyzers
- Add configuration flags

**Testing**:
- `test_semantic_analyzer_runs_control_flow`
- `test_combined_type_and_control_flow_errors`
- `test_control_flow_after_type_checking`
- `test_control_flow_disabled_flag`
- `test_integration_complex_program`
- `test_error_ordering_and_deduplication`

---

### Step 19: Comprehensive Integration Tests
**File**: `tests/test_control_flow_integration.rs`
**Estimated Tests**: 10-12

Real-world scenarios:
- `test_realistic_function_with_multiple_issues`
- `test_web_handler_pattern`
- `test_data_processing_pipeline`
- `test_error_handling_pattern`
- `test_state_machine_pattern`
- `test_recursive_algorithm`
- `test_class_methods_control_flow`
- `test_decorator_with_control_flow`
- `test_comprehension_with_control_flow`
- `test_context_manager_control_flow`

---

### Step 20: Documentation & Finalization
**Files**: `CHANGELOG.md`, `TODO.md`, `README.md`
**Estimated Tests**: 0

Update documentation:
- Add "Control Flow Analysis" section to CHANGELOG
- Mark Phase 2 as 100% complete in TODO
- Update test count in README badge
- Document all new error types
- Add examples of control flow errors

**Testing**: Final validation
- Run full test suite: `cargo test --workspace`
- Verify all tests pass
- Check test count matches expected (~930-980 total)

---

## Summary

**Total Estimated Tests**: 100-150 new tests
**Expected Final Test Count**: ~930-980 tests (830 baseline + 100-150 new)

**Phases**:
1. Infrastructure (4 steps, 8-13 tests)
2. Unreachable Code (4 steps, 32-40 tests)
3. Variable Initialization (3 steps, 24-30 tests)
4. Return Path Validation (3 steps, 24-30 tests)
5. Dead Code Detection (3 steps, 20-26 tests)
6. Integration & Documentation (3 steps, 18-22 tests)

**Testing Strategy**:
- Test after every step
- Run `cargo test --package silk-semantic` after each checkpoint
- Run `cargo test --workspace` at final integration
- Maintain zero failures throughout

**Git Workflow**:
- Create feature branch: `feat/control-flow-analysis` ✅
- Commit after each phase completion
- Detailed commit messages referencing step numbers
- Final PR when all steps complete

---

## Current Status

- [x] Branch created: `feat/control-flow-analysis`
- [ ] **Phase 1**: Infrastructure Setup (Steps 1-4)
  - [x] **Step 1**: Create Control Flow Error Types ✅ (8 tests)
- [ ] **Phase 2**: Unreachable Code Detection (Steps 5-8)
- [ ] **Phase 3**: Variable Initialization Tracking (Steps 9-11)
- [ ] **Phase 4**: Return Path Validation (Steps 12-14)
- [ ] **Phase 5**: Dead Code Detection (Steps 15-17)
- [ ] **Phase 6**: Integration & Documentation (Steps 18-20)

**Next Step**: Step 1 - Create Control Flow Error Types
