# Silk Compiler Architecture

## Overview

Silk is a compiled programming language that transforms Python-compatible syntax into native machine code with C-level performance. This document provides a comprehensive explanation of how the Silk compiler works, from source code to executable binary.

## Compilation Pipeline

The Silk compiler follows a multi-stage pipeline architecture:

```
Source Code (.silk)
      ↓
[1. Lexical Analysis (Lexer)]
      ↓
    Tokens
      ↓
[2. Syntax Analysis (Parser)]
      ↓
Abstract Syntax Tree (AST)
      ↓
[3. Semantic Analysis]
      ↓
Annotated AST + Symbol Tables
      ↓
[4. High-Level IR Generation (HIR)]
      ↓
High-Level Intermediate Representation
      ↓
[5. Mid-Level IR Lowering (MIR)]
      ↓
Mid-Level Intermediate Representation (SSA Form)
      ↓
[6. Optimization Passes]
      ↓
Optimized MIR
      ↓
[7. Low-Level IR Generation (LIR)]
      ↓
Low-Level Intermediate Representation
      ↓
[8. Code Generation (LLVM Backend)]
      ↓
LLVM IR
      ↓
[9. LLVM Optimization]
      ↓
Optimized LLVM IR
      ↓
[10. Machine Code Generation]
      ↓
Assembly (.asm) or Object Code (.o)
      ↓
[11. Linking]
      ↓
Executable Binary
```

---

## Stage 1: Lexical Analysis (Lexer)

### Purpose
Transform raw source code text into a stream of tokens, the smallest meaningful units of the language.

### Input
```python
def greet(name: str) -> str:
    return f"Hello, {name}!"
```

### Process
1. **Character Stream Processing**
   - Read source file as UTF-8 encoded text
   - Track line and column numbers for error reporting
   - Handle different line endings (LF, CRLF)

2. **Token Recognition**
   - Identify keywords: `def`, `return`, `if`, `while`, etc.
   - Recognize operators: `+`, `-`, `*`, `->`, `**`, etc.
   - Parse literals: integers, floats, strings, booleans
   - Extract identifiers: variable names, function names
   - Handle special tokens: indentation, dedentation, newlines

3. **Indentation Handling** (Python-specific)
   - Track indentation level using a stack
   - Generate INDENT tokens when indentation increases
   - Generate DEDENT tokens when indentation decreases
   - Validate consistent indentation (tabs vs spaces)

4. **String Processing**
   - Handle different string formats:
     - Single quotes: `'text'`
     - Double quotes: `"text"`
     - Triple quotes: `'''text'''` or `"""text"""`
     - Raw strings: `r"text"`
     - F-strings: `f"Hello, {name}"`
   - Process escape sequences: `\n`, `\t`, `\\`, etc.

5. **Number Processing**
   - Integer literals: `42`, `0xFF` (hex), `0o77` (octal), `0b1010` (binary)
   - Float literals: `3.14`, `1.5e-10`, `.5`
   - Complex numbers (if supported): `3+4j`

### Output
Stream of tokens with metadata:
```rust
Token { kind: Keyword(Def), span: Span { start: 0, end: 3 }, line: 1, column: 1 }
Token { kind: Identifier("greet"), span: Span { start: 4, end: 9 }, line: 1, column: 5 }
Token { kind: LeftParen, span: Span { start: 9, end: 10 }, line: 1, column: 10 }
Token { kind: Identifier("name"), span: Span { start: 10, end: 14 }, line: 1, column: 11 }
Token { kind: Colon, span: Span { start: 14, end: 15 }, line: 1, column: 15 }
Token { kind: Identifier("str"), span: Span { start: 16, end: 19 }, line: 1, column: 17 }
// ... more tokens
```

### Error Handling
- Invalid characters → Lexical error with position
- Unterminated strings → Error with suggestion to close quote
- Invalid number format → Error with expected format
- Inconsistent indentation → Error showing the conflict

---

## Stage 2: Syntax Analysis (Parser)

### Purpose
Transform flat token stream into a hierarchical Abstract Syntax Tree (AST) that represents the program's structure.

### Input
Token stream from lexer.

### Process

#### 2.1 Recursive Descent Parsing
The parser uses recursive descent with operator precedence for expressions.

**Parsing Strategy:**
```
program → statement*
statement → function_def | class_def | if_stmt | while_stmt | for_stmt | 
            return_stmt | assign_stmt | expr_stmt | import_stmt | ...
expression → assignment_expr
assignment_expr → identifier '=' expression | conditional_expr
conditional_expr → or_expr ('if' or_expr 'else' expression)?
or_expr → and_expr ('or' and_expr)*
and_expr → comparison ('and' comparison)*
comparison → arith_expr (comp_op arith_expr)*
arith_expr → term (('+' | '-') term)*
term → factor (('*' | '/' | '//' | '%') factor)*
factor → ('+' | '-' | '~') factor | power
power → primary ('**' factor)?
primary → identifier | literal | '(' expression ')' | '[' list ']' | ...
```

#### 2.2 Expression Parsing
Uses Pratt parsing (operator precedence parsing) for efficient handling:
- Binary operators: `+`, `-`, `*`, `/`, `**`, `//`, `%`, `&`, `|`, etc.
- Unary operators: `-`, `+`, `~`, `not`
- Comparison chains: `a < b <= c` → `(a < b) and (b <= c)`
- Function calls: `func(arg1, arg2)`
- Indexing: `list[0]`, `dict["key"]`
- Slicing: `list[1:5:2]`
- Attribute access: `obj.method()`

#### 2.3 Statement Parsing
Each statement type has a dedicated parsing function:

**Function Definition:**
```python
def add(x: int, y: int) -> int:
    return x + y
```
→ Parses decorators, name, parameters, return type, and body

**Class Definition:**
```python
class Point:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
```
→ Parses class name, base classes, decorators, and methods

**Control Flow:**
- If/elif/else with multiple branches
- While loops with optional else clause
- For loops with iteration and optional else
- Try/except/finally with multiple exception handlers

#### 2.4 Type Annotation Parsing
```python
x: int = 42
def func(a: list[int], b: dict[str, float]) -> tuple[int, int]:
    ...
```
→ Parses type hints and generic type arguments

### Output (AST)
```rust
Program {
    statements: vec![
        Statement::FunctionDef {
            name: "greet",
            params: vec![
                Parameter {
                    name: "name",
                    type_annotation: Some(Type::Name("str")),
                    default: None,
                }
            ],
            return_type: Some(Type::Name("str")),
            body: vec![
                Statement::Return {
                    value: Some(Expression::FString {
                        parts: vec![
                            FStringPart::Literal("Hello, "),
                            FStringPart::Expression(Expression::Name("name")),
                            FStringPart::Literal("!"),
                        ]
                    })
                }
            ],
            decorators: vec![],
            span: Span { ... },
        }
    ]
}
```

### Error Recovery
- Panic mode recovery: Skip tokens until synchronization point (`;`, `}`, `def`, `class`)
- Error production rules for common mistakes
- Multiple error reporting in single parse
- Suggestion system for common typos

---

## Stage 3: Semantic Analysis

### Purpose
Validate program correctness beyond syntax: type checking, name resolution, and semantic validation.

### Input
Abstract Syntax Tree from parser.

### Process

#### 3.1 Symbol Table Construction
Build hierarchical symbol tables tracking all names:

```rust
struct SymbolTable {
    parent: Option<Box<SymbolTable>>,
    symbols: HashMap<String, Symbol>,
    scope_type: ScopeType, // Global, Function, Class, Block
}

struct Symbol {
    name: String,
    kind: SymbolKind, // Variable, Function, Class, Module
    type_info: Type,
    is_mutable: bool,
    defined_at: Span,
}
```

**Scope Resolution:**
- Global scope: module-level definitions
- Function scope: parameters and local variables
- Class scope: methods and attributes
- Block scope: loop variables, with-statement variables

**Name Resolution Algorithm:**
1. Look up name in current scope
2. If not found, search parent scopes (LEGB rule: Local, Enclosing, Global, Built-in)
3. If still not found, report "name not defined" error
4. Track variable usage for "unused variable" warnings

#### 3.2 Type Inference and Checking

**Type Inference Engine:**
Uses a constraint-based approach similar to Hindley-Milner:

1. **Constraint Generation:**
   ```python
   x = 42          # x: int
   y = x + 1       # y: int (from x: int and 1: int)
   z = "hello"     # z: str
   ```

2. **Type Propagation:**
   ```python
   def add(a, b):  # Initial: a: ?, b: ?, return: ?
       return a + b
   
   result = add(1, 2)  # Infer: a: int, b: int, return: int
   ```

3. **Generic Type Instantiation:**
   ```python
   def first(items: list[T]) -> T:
       return items[0]
   
   x = first([1, 2, 3])  # Instantiate T = int, x: int
   ```

**Type Checking Rules:**
- Assignment compatibility
- Function call argument types match parameters
- Return type matches function signature
- Operator overloading resolution
- Method resolution order (MRO) for inheritance

#### 3.3 Definite Assignment Analysis
Ensure variables are assigned before use:
```python
if condition:
    x = 1
# Error: x might not be assigned here
print(x)
```

Track assignments through all control flow paths.

#### 3.4 Control Flow Analysis
- Ensure `return` statements in all paths for non-None returning functions
- Detect unreachable code:
  ```python
  return x
  print("This will never execute")  # Warning: unreachable code
  ```
- Validate `break`/`continue` only appear inside loops
- Validate `return` only appears inside functions

#### 3.5 Closure Capture Analysis
Identify variables captured from enclosing scopes:
```python
def outer(x):
    def inner(y):
        return x + y  # Capture x from outer scope
    return inner
```
Mark captured variables for special handling in code generation.

#### 3.6 Decorator Validation
Ensure decorators are valid and applicable:
```python
@staticmethod
def method(x):  # Valid
    pass

@property
def value(self):  # Valid for properties
    return self._value
```

### Output
- Annotated AST with type information on every node
- Symbol tables for all scopes
- List of semantic errors and warnings
- Closure capture information

### Error Examples
- `TypeError: Cannot add 'int' and 'str'`
- `NameError: Name 'x' is not defined`
- `TypeError: Function 'add' expects 2 arguments, got 3`
- `AttributeError: Type 'int' has no attribute 'append'`

---

## Stage 4: High-Level IR Generation (HIR)

### Purpose
Transform annotated AST into a platform-independent intermediate representation that's close to the source but easier to optimize.

### Input
Annotated AST with type information.

### HIR Characteristics
- Still high-level (similar to source)
- Preserves Python semantics
- Type information attached to all expressions
- Control flow made more explicit
- Magic methods desugared

### Transformations

#### 4.1 Desugaring
Transform syntactic sugar into explicit operations:

**F-strings:**
```python
f"Hello, {name}!"
```
→
```rust
HIR::Call {
    func: HIR::Builtin(Builtin::StrFormat),
    args: vec![
        HIR::Literal("Hello, "),
        HIR::Call { func: HIR::Builtin(Builtin::Str), args: vec![HIR::Name("name")] },
        HIR::Literal("!"),
    ]
}
```

**List Comprehensions:**
```python
[x * 2 for x in items if x > 0]
```
→
```rust
HIR::Block {
    stmts: vec![
        HIR::LetBinding { name: "$result", value: HIR::ListNew() },
        HIR::ForLoop {
            var: "x",
            iter: HIR::Name("items"),
            body: vec![
                HIR::If {
                    condition: HIR::BinOp {
                        op: Greater,
                        left: HIR::Name("x"),
                        right: HIR::Literal(0),
                    },
                    then_branch: vec![
                        HIR::Call {
                            func: HIR::Attribute {
                                object: HIR::Name("$result"),
                                attr: "append"
                            },
                            args: vec![
                                HIR::BinOp {
                                    op: Multiply,
                                    left: HIR::Name("x"),
                                    right: HIR::Literal(2),
                                }
                            ]
                        }
                    ],
                    else_branch: None,
                }
            ]
        },
        HIR::Name("$result")
    ]
}
```

**Operator Overloading:**
```python
a + b  # where a and b are custom objects
```
→
```rust
HIR::Call {
    func: HIR::Attribute {
        object: HIR::Name("a"),
        attr: "__add__"
    },
    args: vec![HIR::Name("b")]
}
```

#### 4.2 Control Flow Normalization
- Convert `elif` chains to nested if-else
- Make implicit returns explicit
- Normalize loop structures
- Expand `with` statements to try-finally with context manager protocol

#### 4.3 Class Lowering (Initial)
- Separate class definition from instance creation
- Make metaclass calls explicit
- Desugar decorators into function applications
- Create method dispatch tables

### Output Example
```rust
HIR::Module {
    functions: vec![
        HIR::Function {
            name: "greet",
            params: vec![
                HIR::Param { name: "name", ty: Type::Str }
            ],
            return_ty: Type::Str,
            body: HIR::Block {
                stmts: vec![
                    HIR::Return {
                        value: HIR::Call {
                            func: HIR::Builtin(Builtin::StrConcat),
                            args: vec![
                                HIR::Literal("Hello, "),
                                HIR::Name("name"),
                                HIR::Literal("!"),
                            ]
                        }
                    }
                ]
            }
        }
    ]
}
```

---

## Stage 5: Mid-Level IR Lowering (MIR)

### Purpose
Transform HIR into SSA (Static Single Assignment) form with explicit control flow for optimization.

### Input
High-level IR (HIR).

### MIR Characteristics
- **SSA Form**: Each variable assigned exactly once
- **Control Flow Graph (CFG)**: Explicit basic blocks and edges
- **Phi Functions**: Merge values from different control flow paths
- **Explicit Memory Operations**: Load/store made visible
- **Type Information Preserved**: For type-based optimizations

### Transformations

#### 5.1 SSA Conversion
Convert variables to SSA form:

**Before (HIR):**
```python
x = 1
if condition:
    x = 2
else:
    x = 3
return x
```

**After (MIR):**
```rust
bb0:
    x₀ = 1
    br condition, bb1, bb2

bb1:
    x₁ = 2
    jump bb3

bb2:
    x₂ = 3
    jump bb3

bb3:
    x₃ = φ(x₁ from bb1, x₂ from bb2)
    return x₃
```

#### 5.2 Control Flow Graph Construction
Break code into basic blocks:

```rust
struct BasicBlock {
    id: BlockId,
    instructions: Vec<Instruction>,
    terminator: Terminator,
}

enum Terminator {
    Return(Value),
    Jump(BlockId),
    Branch { condition: Value, true_block: BlockId, false_block: BlockId },
    Switch { value: Value, cases: Vec<(Value, BlockId)>, default: BlockId },
}
```

#### 5.3 Memory Operations
Make memory access explicit:

```python
x = obj.field
obj.field = 42
```
→
```rust
// Read
temp₁ = GetAttr(obj, "field")
x₀ = Load(temp₁)

// Write
temp₂ = GetAttr(obj, "field")
Store(temp₂, 42)
```

#### 5.4 Function Calls
Explicit calling convention:

```rust
MIR::Call {
    func: MIR::FunctionRef("greet"),
    args: vec![MIR::Value("name")],
    destination: Some(MIR::LocalRef("result")),
    cleanup: None,  // For exception handling
}
```

### Output Example (MIR)
```rust
Function: greet
Parameters: [%name: str]
Return: str

bb0:
    %0 = StrLiteral("Hello, ")
    %1 = StrLiteral("!")
    %2 = LoadParam(%name)
    %3 = Call(builtin::str_concat, [%0, %2, %1])
    Return(%3)
```

---

## Stage 6: Optimization Passes

### Purpose
Improve performance and reduce code size while preserving semantics.

### Input
MIR in SSA form.

### Optimization Passes

#### 6.1 Dead Code Elimination (DCE)
Remove unused code:
```rust
x₀ = 42        // Used
y₀ = 100       // Never used → Remove
z₀ = x₀ + 1    // Used
return z₀
```

#### 6.2 Constant Folding and Propagation
Evaluate constants at compile time:
```rust
// Before
x₀ = 2
y₀ = 3
z₀ = x₀ * y₀
w₀ = z₀ + 1

// After
w₀ = 7
```

#### 6.3 Common Subexpression Elimination (CSE)
Reuse computed values:
```rust
// Before
a₀ = x + y
b₀ = x + y  // Duplicate computation

// After
a₀ = x + y
b₀ = a₀     // Reuse result
```

#### 6.4 Function Inlining
Replace function calls with function body:
```python
def add(a, b):
    return a + b

x = add(1, 2)
```
→
```rust
// Inlined
x₀ = 1 + 2
x₁ = 3  // After constant folding
```

**Inlining Heuristics:**
- Small functions (< 50 instructions)
- Functions called once
- Hot paths (based on profiling)
- Avoid: recursive functions, large functions

#### 6.5 Loop Optimizations

**Loop Invariant Code Motion (LICM):**
```python
for i in range(n):
    x = expensive_function()  # Doesn't depend on i
    result += i * x
```
→
```python
x = expensive_function()  # Move outside loop
for i in range(n):
    result += i * x
```

**Loop Unrolling:**
```python
for i in range(4):
    process(i)
```
→
```python
process(0)
process(1)
process(2)
process(3)
```

**Strength Reduction:**
```python
for i in range(n):
    x = i * 4  # Multiplication
```
→
```python
x = 0
for i in range(n):
    x = x + 4  # Addition (cheaper)
```

#### 6.6 Type-Based Optimizations

**Devirtualization:**
```python
obj.method()  # Virtual call
```
If type is known at compile time:
```rust
// Direct call instead of virtual dispatch
ClassName::method(obj)
```

**Specialization:**
Create specialized versions for common types:
```python
def add(a, b):
    return a + b
```
Generate:
- `add_int(int, int) -> int` using native integer addition
- `add_float(float, float) -> float` using native float addition
- `add_generic(T, T) -> T` fallback

#### 6.7 Escape Analysis
Determine if objects can be stack-allocated:
```python
def create_point():
    p = Point(1, 2)  # Doesn't escape function
    return p.x + p.y
```
→ Allocate `Point` on stack instead of heap

#### 6.8 Tail Call Optimization
Convert tail-recursive calls to loops:
```python
def factorial(n, acc=1):
    if n == 0:
        return acc
    return factorial(n - 1, n * acc)
```
→
```rust
// Converted to loop
def factorial(n, acc=1):
    loop:
        if n == 0:
            return acc
        n = n - 1
        acc = n * acc
        continue loop
```

### Optimization Order
1. Inlining (enables other optimizations)
2. Constant propagation
3. Dead code elimination
4. CSE
5. Loop optimizations
6. Escape analysis
7. Tail call optimization
8. Final DCE pass

---

## Stage 7: Low-Level IR Generation (LIR)

### Purpose
Transform MIR into a lower-level representation close to machine code.

### Input
Optimized MIR.

### LIR Characteristics
- Target-specific operations
- Register allocation preparation
- Explicit memory layout
- Calling conventions explicit
- Platform-specific instructions

### Transformations

#### 7.1 Lowering Operations
Map high-level operations to low-level equivalents:

**String Operations:**
```rust
// MIR
str_concat(s1, s2)

// LIR (pseudo-code)
len1 = load [s1.length]
len2 = load [s2.length]
new_len = add len1, len2
result = call allocate_string(new_len)
memcpy [result.data], [s1.data], len1
memcpy [result.data + len1], [s2.data], len2
```

**List Operations:**
```rust
// MIR
list.append(item)

// LIR
capacity = load [list.capacity]
length = load [list.length]
if length >= capacity:
    call resize_list(list, capacity * 2)
data = load [list.data]
store [data + length * 8], item
new_length = add length, 1
store [list.length], new_length
```

#### 7.2 Calling Convention
Apply platform-specific ABI:

**x86_64 System V (Linux/macOS):**
- First 6 integer args: RDI, RSI, RDX, RCX, R8, R9
- First 8 float args: XMM0-XMM7
- Return value: RAX (integer), XMM0 (float)
- Caller-saved: RAX, RCX, RDX, RSI, RDI, R8-R11
- Callee-saved: RBX, RBP, R12-R15

**Windows x64:**
- First 4 args: RCX, RDX, R8, R9
- Return value: RAX
- Different caller/callee-saved registers

#### 7.3 Memory Layout
Define data structure layout:
```rust
class Point:
    x: int
    y: int

// Memory layout
[vtable_ptr: 8 bytes]  // For virtual methods
[x: 8 bytes]
[y: 8 bytes]
Total: 24 bytes
```

#### 7.4 Exception Handling
Implement unwinding mechanism:
```rust
// Register cleanup handlers
call __silk_register_cleanup(cleanup_fn)

// On exception
if exception:
    call __silk_unwind()  // Calls cleanup handlers
    jump exception_handler
```

### Output Example (LIR)
```assembly-like
function greet:
.entry:
    ; Prologue
    push rbp
    mov rbp, rsp
    sub rsp, 32          ; Allocate stack space
    
    ; Load parameter
    mov rax, [rbp + 16]  ; Load 'name' parameter
    
    ; Allocate result string
    mov rdi, 7           ; "Hello, " length
    add rdi, [rax]       ; Add name length
    add rdi, 1           ; Add "!" length
    call allocate_string
    mov [rbp - 8], rax   ; Store result
    
    ; Copy "Hello, "
    mov rdi, rax
    lea rsi, [.str_hello]
    mov rdx, 7
    call memcpy
    
    ; Copy name
    mov rdi, [rbp - 8]
    add rdi, 7
    mov rsi, [rbp + 16]
    mov rdx, [rsi]       ; Name length
    call memcpy
    
    ; Copy "!"
    ; ... (similar)
    
    ; Return
    mov rax, [rbp - 8]
    mov rsp, rbp
    pop rbp
    ret

.str_hello:
    .string "Hello, "
```

---

## Stage 8: Code Generation (LLVM Backend)

### Purpose
Generate LLVM IR from LIR, leveraging LLVM's optimization and code generation.

### Input
Low-level IR (LIR).

### Process

#### 8.1 LLVM IR Generation
Translate LIR to LLVM IR:

```llvm
define dso_local ptr @greet(ptr %name) {
entry:
  ; Get string lengths
  %name.len.ptr = getelementptr inbounds %String, ptr %name, i32 0, i32 0
  %name.len = load i64, ptr %name.len.ptr
  
  ; Calculate result length
  %hello.len = i64 7
  %exclaim.len = i64 1
  %tmp1 = add i64 %hello.len, %name.len
  %result.len = add i64 %tmp1, %exclaim.len
  
  ; Allocate result string
  %result = call ptr @allocate_string(i64 %result.len)
  %result.data = getelementptr inbounds %String, ptr %result, i32 0, i32 1
  
  ; Copy "Hello, "
  %hello.data = getelementptr [7 x i8], ptr @.str.hello, i64 0, i64 0
  call void @llvm.memcpy.p0.p0.i64(ptr %result.data, ptr %hello.data, i64 7, i1 false)
  
  ; Copy name
  %name.data = getelementptr inbounds %String, ptr %name, i32 0, i32 1
  %dest1 = getelementptr i8, ptr %result.data, i64 7
  call void @llvm.memcpy.p0.p0.i64(ptr %dest1, ptr %name.data, i64 %name.len, i1 false)
  
  ; Copy "!"
  %exclaim.data = getelementptr [1 x i8], ptr @.str.exclaim, i64 0, i64 0
  %offset2 = add i64 7, %name.len
  %dest2 = getelementptr i8, ptr %result.data, i64 %offset2
  call void @llvm.memcpy.p0.p0.i64(ptr %dest2, ptr %exclaim.data, i64 1, i1 false)
  
  ret ptr %result
}

@.str.hello = private unnamed_addr constant [7 x i8] c"Hello, "
@.str.exclaim = private unnamed_addr constant [1 x i8] c"!"
```

#### 8.2 Type Mapping
Map Silk types to LLVM types:
- `int` → `i64` (64-bit integer)
- `float` → `double` (64-bit float)
- `bool` → `i1` (1-bit integer)
- `str` → `ptr` (pointer to String struct)
- `list[T]` → `ptr` (pointer to List struct)
- `dict[K, V]` → `ptr` (pointer to Dict struct)

#### 8.3 Runtime Library Linking
Generate calls to runtime functions:
- Memory allocation: `@allocate_string`, `@allocate_list`, etc.
- Reference counting: `@incref`, `@decref`
- Type operations: `@isinstance`, `@type_name`
- Built-in functions: `@builtin_print`, `@builtin_len`, etc.

#### 8.4 Debug Information
Generate DWARF debug info:
```llvm
!1 = !DIFile(filename: "example.silk", directory: "/path/to/project")
!2 = !DISubroutineType(types: !3)
!3 = !{!4, !4}  ; Return type, parameter types
!4 = !DIBasicType(name: "str", size: 64, encoding: DW_ATE_address)
!5 = distinct !DISubprogram(
  name: "greet",
  scope: !1,
  file: !1,
  line: 1,
  type: !2,
  scopeLine: 1,
  spFlags: DISPFlagDefinition,
  unit: !0
)
```

---

## Stage 9: LLVM Optimization

### Purpose
Apply LLVM's extensive optimization passes for maximum performance.

### LLVM Optimization Pipeline

#### 9.1 Module-Level Optimizations
- **Global Dead Code Elimination**: Remove unused functions
- **Inlining**: Aggressive function inlining
- **Interprocedural Constant Propagation**: Propagate constants across functions
- **Global Optimizer**: Optimize global variables

#### 9.2 Function-Level Optimizations
- **Instruction Combining**: Simplify instruction sequences
- **CFG Simplification**: Merge basic blocks, remove unreachable code
- **Scalar Replacement of Aggregates (SROA)**: Break structs into scalars
- **Early CSE**: Common subexpression elimination
- **Jump Threading**: Duplicate blocks to enable better optimization

#### 9.3 Loop Optimizations
- **Loop Invariant Code Motion**
- **Loop Unrolling**
- **Loop Vectorization**: Use SIMD instructions
- **Loop Deletion**: Remove empty loops
- **Induction Variable Simplification**

#### 9.4 Target-Specific Optimizations
- **Instruction selection**: Choose optimal instructions
- **Instruction scheduling**: Reorder for better performance
- **Register pressure reduction**
- **Peephole optimizations**: Local instruction improvements

### Optimization Levels
- **-O0**: No optimization (fast compilation, debugging)
- **-O1**: Basic optimizations (balanced)
- **-O2**: Aggressive optimizations (recommended for release)
- **-O3**: Maximum optimizations (may increase code size)
- **-Os**: Optimize for size
- **-Oz**: Aggressively optimize for size

---

## Stage 10: Machine Code Generation

### Purpose
Generate native assembly or object code for the target platform.

### Input
Optimized LLVM IR.

### Process

#### 10.1 Target Selection
Select target triple (arch-vendor-os):
- `x86_64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `aarch64-apple-darwin` (Apple Silicon)
- `wasm32-unknown-unknown` (WebAssembly)

#### 10.2 Instruction Selection
Map LLVM IR to target machine instructions:

**LLVM IR:**
```llvm
%add = add i64 %a, %b
```

**x86_64 Assembly:**
```assembly
mov rax, [rbp-8]    ; Load %a
mov rbx, [rbp-16]   ; Load %b
add rax, rbx        ; Add
mov [rbp-24], rax   ; Store result
```

**ARM64 Assembly:**
```assembly
ldr x0, [sp, #8]    ; Load %a
ldr x1, [sp, #16]   ; Load %b
add x0, x0, x1      ; Add
str x0, [sp, #24]   ; Store result
```

#### 10.3 Register Allocation
Assign virtual registers to physical registers:

**Algorithms:**
- **Linear Scan**: Fast, good for JIT
- **Graph Coloring**: Better quality, slower
- **Greedy Allocation**: Balance between speed and quality

**Spilling:**
When not enough registers, store values to stack:
```assembly
; Out of registers, spill to stack
mov [rsp-8], rax
; ... use rax for something else ...
mov rax, [rsp-8]    ; Reload when needed
```

#### 10.4 Instruction Scheduling
Reorder instructions to minimize pipeline stalls:
```assembly
; Before (pipeline stall)
load rax, [mem]
add rbx, rax       ; Stalls waiting for load

; After (no stall)
load rax, [mem]
mov rcx, rdx       ; Independent instruction
add rbx, rax       ; Load completed
```

#### 10.5 Assembly Generation
Generate human-readable assembly (optional):
```assembly
    .section    __TEXT,__text,regular,pure_instructions
    .globl  _greet
    .p2align    4, 0x90
_greet:
    push    rbp
    mov     rbp, rsp
    sub     rsp, 32
    mov     qword ptr [rbp - 8], rdi
    ; ... function body ...
    add     rsp, 32
    pop     rbp
    ret
```

#### 10.6 Object Code Generation
Generate binary object file (.o or .obj):
- Machine code bytes
- Relocation information
- Symbol table
- Debug information (DWARF/PDB)

---

## Stage 11: Linking

### Purpose
Combine object files and libraries into final executable.

### Input
- Object file(s) from code generation
- Silk runtime library
- System libraries (libc, libm, etc.)

### Process

#### 11.1 Symbol Resolution
Resolve references between object files:
```
main.o:
  calls: greet (undefined)

greet.o:
  defines: greet
```
Linker connects the call in main.o to the definition in greet.o

#### 11.2 Relocation
Adjust addresses for final memory layout:
```assembly
; Before linking (relative address)
call greet  ; call to offset +???

; After linking (absolute address)
call 0x100001234  ; Actual address in memory
```

#### 11.3 Library Linking

**Static Linking:**
Copy library code into executable:
```bash
silkc main.silk --link-static
# Produces large, self-contained executable
```

**Dynamic Linking:**
Reference shared libraries:
```bash
silkc main.silk --link-dynamic
# Produces smaller executable, requires .so/.dylib/.dll at runtime
```

#### 11.4 Executable Generation
Create final executable with:
- **Text segment**: Machine code
- **Data segment**: Global variables, string literals
- **BSS segment**: Uninitialized data
- **Symbol table**: For debugging
- **Relocation table**: For dynamic linking

**Linux ELF Format:**
```
ELF Header
Program Headers
.text section     (executable code)
.data section     (initialized data)
.rodata section   (read-only data)
.bss section      (uninitialized data)
.symtab section   (symbol table)
.strtab section   (string table)
.debug_* sections (debug information)
Section Headers
```

**macOS Mach-O Format:**
```
Mach Header
Load Commands
__TEXT segment
  __text section    (code)
  __const section   (constants)
__DATA segment
  __data section    (data)
  __bss section     (bss)
Symbol Table
String Table
```

**Windows PE Format:**
```
DOS Header
PE Header
Section Table
.text section     (code)
.data section     (data)
.rdata section    (read-only data)
.bss section      (bss)
Import Table
Export Table
Debug Directory
```

### Output
Native executable binary ready to run.

---

## Execution

### Program Startup

1. **OS Loader**
   - Load executable into memory
   - Map segments to virtual memory
   - Load dynamic libraries
   - Initialize process

2. **Runtime Initialization**
   ```c
   // Silk runtime initialization
   void __silk_init() {
       initialize_memory_allocator();
       initialize_exception_handler();
       register_signal_handlers();
       setup_thread_local_storage();
   }
   ```

3. **Main Function**
   ```assembly
   _start:
       call __silk_init
       call main           ; User's main function
       mov rdi, rax        ; Exit code
       call exit
   ```

4. **Runtime Support**
   - Memory management (allocation/deallocation)
   - Reference counting (if used)
   - Exception handling
   - Type reflection
   - Built-in functions

5. **Program Termination**
   ```c
   void __silk_cleanup() {
       run_finalizers();
       cleanup_memory();
       close_file_handles();
   }
   ```

---

## Performance Characteristics

### Compilation Speed
- **Lexer**: ~1-2 MB/s
- **Parser**: ~500 KB/s - 1 MB/s
- **Semantic Analysis**: ~200-500 KB/s
- **Optimization**: ~100-300 KB/s (depends on level)
- **Code Generation**: ~50-200 KB/s
- **Overall**: ~1-5 seconds for medium projects (10K-50K LOC)

### Runtime Performance
- **Native integers**: Same as C (no boxing)
- **Function calls**: Direct calls, no overhead
- **Memory access**: Direct pointers, cache-friendly
- **Target**: 50-100x faster than CPython
- **Target**: Within 10% of hand-written C

### Memory Usage
- **Compile-time**: ~100-500 MB for medium projects
- **Runtime**: Minimal overhead, no GC pauses
- **Objects**: Compact layout, efficient packing

---

## Example End-to-End Compilation

### Source Code (hello.silk)
```python
def greet(name: str) -> str:
    return f"Hello, {name}!"

def main():
    message = greet("World")
    print(message)

if __name__ == "__main__":
    main()
```

### Compilation Command
```bash
silkc hello.silk -o hello --opt-level 2
```

### Compilation Steps
1. **Lexer**: ~1ms (53 tokens)
2. **Parser**: ~2ms (AST with 12 nodes)
3. **Semantic**: ~5ms (type checking, 2 functions)
4. **HIR Gen**: ~3ms
5. **MIR Gen**: ~4ms (15 basic blocks)
6. **Optimize**: ~20ms (3 optimization passes)
7. **LIR Gen**: ~5ms
8. **LLVM IR**: ~10ms
9. **LLVM Opt**: ~50ms
10. **Codegen**: ~30ms
11. **Link**: ~20ms

**Total**: ~150ms

### Output
```bash
$ ./hello
Hello, World!
```

---

## Summary

The Silk compiler transforms Python-syntax source code through 11 distinct stages:

1. **Lexing** → Tokens
2. **Parsing** → AST
3. **Semantic Analysis** → Typed AST
4. **HIR Generation** → High-level IR
5. **MIR Lowering** → SSA form with CFG
6. **Optimization** → Optimized MIR
7. **LIR Generation** → Low-level IR
8. **LLVM IR Generation** → LLVM IR
9. **LLVM Optimization** → Optimized LLVM IR
10. **Code Generation** → Assembly/Object code
11. **Linking** → Executable

Each stage builds upon the previous, gradually lowering the abstraction level from Python syntax to machine code, while preserving semantics and applying aggressive optimizations for C-level performance.
