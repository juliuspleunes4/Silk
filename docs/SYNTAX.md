# Silk Language Syntax Reference

> **Implementation Status**: Frontend (lexer and parser) is **100% complete**. All syntax documented here is fully parsed and validated by **467 comprehensive tests** (115 lexer + 255 parser + 97 semantic analyzer tests). Type checking and code generation are in progress but not yet complete.

## Table of Contents

1. [Literals](#literals)
   - [Integer Literals](#integer-literals)
   - [Float Literals](#float-literals)
   - [String Literals](#string-literals)
   - [Boolean and None Literals](#boolean-and-none-literals)
   - [Special Literals](#special-literals)
2. [Identifiers and Keywords](#identifiers-and-keywords)
3. [Operators](#operators)
   - [Arithmetic Operators](#arithmetic-operators)
   - [Comparison Operators](#comparison-operators)
   - [Logical Operators](#logical-operators)
   - [Bitwise Operators](#bitwise-operators)
   - [Assignment Operators](#assignment-operators)
4. [Expressions](#expressions)
   - [Binary Operations](#binary-operations)
   - [Unary Operations](#unary-operations)
   - [Comparison Chains](#comparison-chains)
   - [Function Calls](#function-calls)
   - [Subscripting and Slicing](#subscripting-and-slicing)
   - [Attribute Access](#attribute-access)
   - [Lambda Expressions](#lambda-expressions)
   - [Conditional Expressions](#conditional-expressions)
   - [Walrus Operator](#walrus-operator)
5. [Collection Literals](#collection-literals)
   - [Lists](#lists)
   - [Tuples](#tuples)
   - [Dictionaries](#dictionaries)
   - [Sets](#sets)
6. [Comprehensions](#comprehensions)
   - [List Comprehensions](#list-comprehensions)
   - [Dictionary Comprehensions](#dictionary-comprehensions)
   - [Set Comprehensions](#set-comprehensions)
   - [Generator Expressions](#generator-expressions)
7. [Statements](#statements)
   - [Expression Statements](#expression-statements)
   - [Assignment Statements](#assignment-statements)
   - [Control Flow](#control-flow)
   - [Function Definitions](#function-definitions)
   - [Class Definitions](#class-definitions)
   - [Import Statements](#import-statements)
   - [Exception Handling](#exception-handling)
   - [Context Managers](#context-managers)
   - [Pattern Matching](#pattern-matching)
8. [Type Annotations](#type-annotations)
9. [Decorators](#decorators)
10. [Important Notes](#important-notes)

---

## Literals

### Integer Literals

Silk supports multiple integer formats with optional underscores for readability:

#### Decimal Integers
```python
42
1_000_000  # Underscores for readability
0          # Zero
```

#### Binary Integers (prefix: `0b` or `0B`)
```python
0b1010       # Binary: 10 in decimal
0b1111_0000  # Binary with underscores: 240 in decimal
0B1111       # Uppercase B: 15 in decimal
```

#### Octal Integers (prefix: `0o` or `0O`)
```python
0o755        # Octal: 493 in decimal
0o77_77      # Octal with underscores: 4095 in decimal
0O10         # Uppercase O: 8 in decimal
```

#### Hexadecimal Integers (prefix: `0x` or `0X`)
```python
0xFF         # Hex: 255 in decimal
0xDEAD_BEEF  # Hex with underscores: 3735928559 in decimal
0X1A2B       # Uppercase X: 6699 in decimal
0xDeAdBeEf   # Mixed case digits
```

### Float Literals

Floating-point numbers support decimal notation and scientific notation:

```python
3.14
0.5
10.0
1_000.5      # Underscores allowed

# Scientific notation
1e10         # 1 × 10^10
3.14e-2      # 0.0314
1.5e+3       # 1500.0
2.5e10
1_000.0e3    # Underscores in mantissa
```

### String Literals

Silk provides comprehensive string literal support with multiple formats:

#### Basic Strings (single or double quotes)
```python
"Hello, World!"
'Hello, World!'
"String with 'single' quotes inside"
'String with "double" quotes inside'
```

#### Triple-Quoted Strings (multiline)
```python
"""
This is a multiline string.
It can span multiple lines.
"""

'''
Another multiline string
with single quotes.
'''
```

#### Escape Sequences
Standard Python escape sequences are supported in regular strings:
```python
"Line 1\nLine 2"    # Newline
"Tab\tseparated"    # Tab
"Quote: \"text\""   # Escaped quote
"Backslash: \\"     # Escaped backslash
"Unicode: \u0041"   # Unicode escape (A)
"Hex: \x41"         # Hex escape (A)
```

#### Raw Strings (prefix: `r` or `R`)
Escape sequences are preserved as literal text:
```python
r"C:\Users\name\file.txt"     # Windows path - backslashes not escaped
r"\n is not a newline"         # \n treated as literal characters
r"Regex: \d+\.\d+"             # Regex patterns
r'''Raw triple-quoted string''' # Also works with triple quotes
R"Uppercase R prefix"          # Uppercase R also works
```

#### F-Strings (prefix: `f` or `F`)
Formatted string literals with embedded expressions:
```python
f"Hello, {name}!"
f"Result: {x + y}"
f"Value: {value:.2f}"          # With format specifier
f"Multiple: {a} and {b}"       # Multiple expressions
f"Expression: {foo(x, y)}"     # Function calls in expressions
f"Escaped braces: {{not an expression}}"  # Literal braces
F"Uppercase F prefix: {value}"
```

F-string expressions can contain:
- Variables: `{name}`
- Arithmetic: `{x + y * 2}`
- Function calls: `{func(arg)}`
- Attribute access: `{obj.attr}`
- Format specifiers: `{value:.2f}`, `{num:04d}`

#### Byte Strings (prefix: `b` or `B`)
Byte literals for binary data:
```python
b"bytes"
b"Data: \x00\x01\x02"          # Hex escapes
b'Single quotes work too'
b"""Triple-quoted byte string"""
B"Uppercase B prefix"
```

Note: Byte strings must contain only ASCII characters (0-127).

#### Byte Raw Strings (prefix: `br`/`rb` or `BR`/`RB`)
Combination of byte and raw string prefixes:
```python
br"C:\path\to\file"            # Windows path as bytes
rb"Same as br"                 # Order doesn't matter
br"\x00\x01"                   # Hex notation preserved
BR"Uppercase BR"
RB"Uppercase RB"
```

### Boolean and None Literals

```python
True    # Boolean true (capitalized)
False   # Boolean false (capitalized)
None    # Null/nil value (capitalized)
```

### Special Literals

#### Ellipsis (`...`)
Used in type hints, stubs, and slice notation:
```python
...                    # Ellipsis literal
def stub_function(): ...  # Function stub
x: tuple[int, ...]     # Variable-length tuple type
matrix[..., 0]         # Advanced slicing
```

#### NotImplemented
Singleton for rich comparison methods:
```python
NotImplemented         # Special value for comparison operators
```

---

## Identifiers and Keywords

### Identifiers

Valid identifier naming rules:
- Start with letter (a-z, A-Z) or underscore (_)
- Followed by letters, digits (0-9), or underscores
- Case-sensitive
- Unicode characters supported

```python
name
_private
__dunder__
snake_case
camelCase
CONSTANT
myVar123
αβγ          # Unicode identifiers
```

### Keywords

Reserved words that cannot be used as identifiers:

```python
# Control flow
if elif else
while for break continue
match case

# Function and class
def class lambda
return yield
async await

# Import and module
import from as

# Exception handling
try except finally raise assert

# Context managers
with

# Variable scope
global nonlocal

# Operators and literals
and or not in is
True False None NotImplemented

# Other
pass del
```

---

## Operators

### Arithmetic Operators

```python
# Basic arithmetic
+     # Addition
-     # Subtraction
*     # Multiplication
/     # Division (always returns float)
//    # Floor division (integer division)
%     # Modulo (remainder)
**    # Exponentiation (power)

# Examples
5 + 3         # 8
10 - 4        # 6
3 * 4         # 12
7 / 2         # 3.5
7 // 2        # 3 (floor division)
7 % 3         # 1 (remainder)
2 ** 8        # 256 (2 to the power 8)
```

### Comparison Operators

```python
==    # Equal to
!=    # Not equal to
<     # Less than
>     # Greater than
<=    # Less than or equal
>=    # Greater than or equal
is    # Identity comparison
is not   # Negated identity
in    # Membership test
not in   # Negated membership

# Examples
x == y
a != b
5 < 10
age >= 18
x is None
key in dictionary
value not in list
```

### Logical Operators

```python
and   # Logical AND
or    # Logical OR
not   # Logical NOT

# Examples
x > 0 and y > 0      # Both conditions must be true
x < 0 or y < 0       # At least one condition must be true
not finished         # Negation
```

### Bitwise Operators

```python
&     # Bitwise AND
|     # Bitwise OR
^     # Bitwise XOR
~     # Bitwise NOT (inversion)
<<    # Left shift
>>    # Right shift

# Examples
0b1010 & 0b1100    # 0b1000 (8)
0b1010 | 0b0101    # 0b1111 (15)
0b1010 ^ 0b1100    # 0b0110 (6)
~0b1010            # Inverts all bits
8 << 2             # 32 (shift left by 2)
32 >> 2            # 8 (shift right by 2)
```

### Assignment Operators

```python
=      # Simple assignment
+=     # Add and assign
-=     # Subtract and assign
*=     # Multiply and assign
/=     # Divide and assign
//=    # Floor divide and assign
%=     # Modulo and assign
**=    # Exponentiate and assign
&=     # Bitwise AND and assign
|=     # Bitwise OR and assign
^=     # Bitwise XOR and assign
<<=    # Left shift and assign
>>=    # Right shift and assign

# Examples
x = 10
x += 5      # x = x + 5  (x is now 15)
x *= 2      # x = x * 2  (x is now 30)
x //= 4     # x = x // 4 (x is now 7)
```

### Operator Precedence

From highest to lowest precedence:

1. `**` (exponentiation) - right-associative
2. `+x`, `-x`, `~x` (unary plus, minus, bitwise NOT)
3. `*`, `/`, `//`, `%` (multiplication, division, modulo)
4. `+`, `-` (addition, subtraction)
5. `<<`, `>>` (bit shifts)
6. `&` (bitwise AND)
7. `^` (bitwise XOR)
8. `|` (bitwise OR)
9. `==`, `!=`, `<`, `>`, `<=`, `>=`, `is`, `is not`, `in`, `not in` (comparisons)
10. `not` (logical NOT)
11. `and` (logical AND)
12. `or` (logical OR)
13. `if`-`else` (conditional expression)
14. `:=` (walrus operator)

---

## Expressions

### Binary Operations

Combine two values with an operator:

```python
# Arithmetic
a + b
x - y
width * height
total / count
base ** exponent

# String concatenation
"Hello, " + "World!"

# Complex expressions with precedence
result = (a + b) * (c - d)
value = x ** 2 + y ** 2
```

### Unary Operations

Single operand operations:

```python
# Arithmetic
+x        # Unary plus (typically no effect)
-x        # Negation
-42       # Negative literal

# Bitwise
~flags    # Bitwise NOT/inversion

# Logical
not condition    # Logical negation
not (x > 0)
```

### Comparison Chains

Python-style comparison chaining:

```python
# Chained comparisons
0 <= x < 100        # Equivalent to: (0 <= x) and (x < 100)
a < b < c           # Equivalent to: (a < b) and (b < c)
a == b == c         # All equal
x < y <= z

# Mixed operators in chain
min_val < value < max_val
a <= b < c <= d
```

### Function Calls

Invoke functions with positional and keyword arguments:

```python
# No arguments
foo()
obj.method()

# Positional arguments
print("Hello")
max(1, 2, 3)

# Keyword arguments
greet(name="Alice", age=30)
connect(host="localhost", port=8080)

# Mixed positional and keyword (positional must come first)
plot(data, title="Results", color="blue")

# Star expressions for unpacking
func(*args, **kwargs)
send(1, 2, *more_args)
configure(x=10, **options)
```

**Important**: Positional arguments must come before keyword arguments.

### Subscripting and Slicing

Access elements and subsequences:

#### Indexing
```python
# Single element access
list[0]           # First element
list[-1]          # Last element
matrix[i][j]      # Nested indexing
dict[key]         # Dictionary access
```

#### Slicing
Full slice syntax: `[start:stop:step]`

```python
# Basic slicing
list[1:5]         # Elements from index 1 to 4
list[:5]          # First 5 elements
list[5:]          # From index 5 to end
list[:]           # Entire list (shallow copy)

# With step
list[::2]         # Every second element
list[1::2]        # Every second element starting from index 1
list[::-1]        # Reverse the list (step=-1)

# Negative indices
list[-5:-1]       # Last 5 elements except the very last
list[-10:]        # Last 10 elements

# Complex slicing
list[start:stop:step]
matrix[0][1:3]    # Slice on nested subscript
```

### Attribute Access

Access object attributes and methods:

```python
# Attribute access
obj.attribute
person.name
config.database.host

# Method calls
obj.method()
text.lower()
list.append(item)

# Chained access
user.profile.settings.theme
data.strip().split(",")
```

### Lambda Expressions

Anonymous inline functions:

```python
# Basic lambda
lambda: 42                    # No parameters
lambda x: x + 1              # Single parameter
lambda x, y: x * y           # Multiple parameters
lambda x, y, z: x + y + z    # Three parameters

# Lambda with complex body
lambda x: x * 2 + 1
lambda name: "Hello, " + name
lambda x: x > 0

# Lambda in function calls
map(lambda x: x * 2, numbers)
sorted(items, key=lambda x: x.name)
filter(lambda x: x > 0, values)

# Lambda in data structures
ops = [lambda x: x + 1, lambda x: x * 2]
config = {"transform": lambda x: x.lower()}
```

**Note**: Lambda body is a single expression (no statements).

### Conditional Expressions

Ternary operator (inline if-else):

```python
# Basic ternary
value if condition else default
x if x > 0 else 0

# With literals
1 if True else 0
"yes" if flag else "no"

# With expressions
x + 1 if x > 0 else x - 1
positive if x > 0 else negative

# With comparisons
"positive" if x > 0 else "non-positive"

# With function calls
foo() if condition else bar()

# Nested ternary
a if x > 0 else b if x < 0 else c

# In function calls
result = compute(x if x > 0 else 0)

# In lists
values = [x if x > 0 else 0 for x in data]
```

### Walrus Operator

Named expressions (assignment within expressions):

```python
# Assign and use in same expression
(n := len(data))
(match := pattern.search(text))

# In if statements
if (n := len(data)) > 10:
    print(f"Large dataset: {n} items")

# In while loops
while (line := file.readline()):
    process(line)

# In list comprehensions
[y for x in data if (y := transform(x)) is not None]

# In function calls
compute(result := expensive_operation())

# With comparisons
if (value := get_value()) > threshold:
    use(value)
```

---

## Collection Literals

### Lists

Ordered, mutable sequences:

```python
# Empty list
[]

# List with elements
[1, 2, 3, 4, 5]
["apple", "banana", "cherry"]

# Mixed types
[1, "two", 3.0, True, None]

# Nested lists
[[1, 2], [3, 4], [5, 6]]
matrix = [[0, 1], [2, 3]]

# With expressions
[x + 1, y * 2, z ** 3]
[foo(), bar(), baz()]

# Trailing comma allowed
[1, 2, 3,]
```

### Tuples

Ordered, immutable sequences:

```python
# Empty tuple
()

# Single-element tuple (note the comma)
(1,)
(x,)

# Multiple elements
(1, 2, 3)
("Alice", 30, True)

# Parentheses optional for multiple elements
x = 1, 2, 3
point = x, y

# Nested tuples
((1, 2), (3, 4))

# Mixed types
(42, "text", 3.14, None)

# Tuple unpacking
x, y = (10, 20)
a, b, c = 1, 2, 3
```

### Dictionaries

Key-value mappings:

```python
# Empty dictionary
{}

# Dictionary with items
{"name": "Alice", "age": 30}
{1: "one", 2: "two", 3: "three"}

# Mixed key and value types
{"a": 1, "b": 2.5, "c": True}
{1: "int key", "2": "str key"}

# With expressions
{key: value, key2: compute(x)}
{"result": x + y, "status": "ok"}

# Nested dictionaries
{
    "user": {
        "name": "Alice",
        "settings": {"theme": "dark"}
    }
}

# Trailing comma allowed
{"a": 1, "b": 2,}
```

### Sets

Unordered collections of unique elements:

```python
# Empty set (must use set() function, {} is an empty dict)
set()

# Set with elements (using curly braces)
{1, 2, 3, 4, 5}
{"apple", "banana", "cherry"}

# Automatically removes duplicates
{1, 2, 2, 3, 3, 3}    # Results in {1, 2, 3}

# With expressions
{x, y, z}
{foo(), bar()}

# Nested? No - sets cannot contain mutable objects
# {1, 2, {3, 4}}  # ERROR: sets are not hashable
```

---

## Comprehensions

### List Comprehensions

Create lists using concise syntax:

```python
# Basic list comprehension
[x for x in range(10)]
[x * 2 for x in numbers]

# With condition (filter)
[x for x in range(10) if x % 2 == 0]    # Even numbers
[x for x in values if x > 0]             # Positive values

# With transformation and filter
[x * 2 for x in range(10) if x % 2 == 0]

# Multiple conditions
[x for x in range(100) if x % 2 == 0 if x % 3 == 0]

# Nested loops
[(x, y) for x in range(3) for y in range(3)]
[x * y for x in range(1, 4) for y in range(1, 4)]

# Nested with filter
[(x, y) for x in range(10) for y in range(10) if x < y]

# Complex expressions
[x.upper() for x in names if len(x) > 3]
[item["value"] for item in data if item["active"]]
```

### Dictionary Comprehensions

Create dictionaries using concise syntax:

```python
# Basic dict comprehension
{x: x ** 2 for x in range(5)}
{word: len(word) for word in words}

# With condition
{x: x ** 2 for x in range(10) if x % 2 == 0}

# Transform keys and values
{k.lower(): v * 2 for k, v in items}
{x: str(x) for x in numbers}

# From two lists
{k: v for k, v in zip(keys, values)}

# Nested loops
{f"{x}-{y}": x * y for x in range(3) for y in range(3)}

# Complex expressions
{key: process(value) for key, value in data.items() if is_valid(key)}
```

### Set Comprehensions

Create sets using concise syntax:

```python
# Basic set comprehension
{x for x in range(10)}
{x * 2 for x in numbers}

# With condition
{x for x in range(20) if x % 2 == 0}
{word.lower() for word in text.split() if len(word) > 3}

# Automatically deduplicates
{x % 5 for x in range(20)}    # {0, 1, 2, 3, 4}

# Nested loops
{x * y for x in range(1, 5) for y in range(1, 5)}

# Complex expressions
{normalize(item) for item in data if is_valid(item)}
```

### Generator Expressions

Create generators (lazy iterators) using comprehension syntax:

```python
# Basic generator expression
(x for x in range(10))
(x * 2 for x in numbers)

# With condition
(x for x in range(100) if x % 2 == 0)

# Memory efficient for large sequences
sum(x ** 2 for x in range(1000000))

# In function calls (parentheses optional when sole argument)
sum(x for x in range(10))
max(len(word) for word in words)

# Must use parentheses with multiple arguments
process(data, (x for x in items if x > 0))

# Nested loops
((x, y) for x in range(10) for y in range(10) if x != y)

# Complex expressions
(item.value for item in data if item.active and item.value > 0)
```

**Comprehension Features**:
- Single or multiple generators: `for x in seq1 for y in seq2`
- Multiple filters: `if cond1 if cond2`
- Async comprehensions: `async for x in async_seq` (parsed but not type-checked yet)

---

## Statements

### Expression Statements

Any expression can be a statement:

```python
# Function calls
print("Hello")
process_data()

# Method calls
list.append(item)
file.close()

# Arithmetic (typically side effects)
x + y    # Valid but result is discarded
```

### Assignment Statements

#### Simple Assignment
```python
# Single assignment
x = 10
name = "Alice"
result = compute()

# Multiple targets (same value)
x = y = z = 0
a = b = c = []

# Type annotation (assignment)
x: int = 10
name: str = "Alice"
data: list[int] = [1, 2, 3]

# Annotated without value
x: int
name: str
```

#### Tuple Unpacking
```python
# Unpack tuple
x, y = (10, 20)
a, b, c = 1, 2, 3

# Swap values
x, y = y, x

# Nested unpacking
(a, b), (c, d) = (1, 2), (3, 4)

# With list
first, second, third = [1, 2, 3]
```

#### Augmented Assignment
```python
x += 5      # x = x + 5
y -= 3      # y = y - 3
z *= 2      # z = z * 2
count += 1
total -= discount
size *= scale
```

### Control Flow

#### If Statements
```python
# Basic if
if condition:
    do_something()

# If-else
if x > 0:
    print("positive")
else:
    print("non-positive")

# If-elif-else
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

# Multiple elif
if grade >= 90:
    letter = "A"
elif grade >= 80:
    letter = "B"
elif grade >= 70:
    letter = "C"
else:
    letter = "F"

# Nested if
if x > 0:
    if x % 2 == 0:
        print("positive even")
    else:
        print("positive odd")
```

#### While Loops
```python
# Basic while
while condition:
    do_something()

# While with counter
count = 0
while count < 10:
    print(count)
    count += 1

# While-else (else runs if loop completes without break)
while searching:
    if found:
        break
else:
    print("Not found")

# Infinite loop
while True:
    if should_stop:
        break
```

#### For Loops
```python
# Basic for loop
for item in sequence:
    process(item)

# With range
for i in range(10):
    print(i)

# Over list
for name in names:
    print(name)

# With tuple unpacking
for key, value in items:
    print(f"{key}: {value}")

# For-else (else runs if loop completes without break)
for item in items:
    if item == target:
        break
else:
    print("Target not found")

# Nested for loops
for i in range(3):
    for j in range(3):
        print(i, j)
```

#### Break and Continue
```python
# Break - exit loop immediately
for i in range(10):
    if i == 5:
        break
    print(i)    # Prints 0-4

# Continue - skip to next iteration
for i in range(10):
    if i % 2 == 0:
        continue
    print(i)    # Prints odd numbers only

# In nested loops (breaks inner loop only)
for i in range(5):
    for j in range(5):
        if j == 3:
            break
        print(i, j)
```

#### Pass Statement
```python
# Placeholder for empty blocks
if condition:
    pass    # TODO: implement later

def empty_function():
    pass    # Function stub

class EmptyClass:
    pass    # Class stub
```

#### Return Statement
```python
# Return from function
def get_value():
    return 42

# Return with expression
def add(a, b):
    return a + b

# Early return
def process(x):
    if x < 0:
        return None
    return x * 2

# Return nothing (implicit None)
def log(message):
    print(message)
    return
```

### Function Definitions

#### Basic Functions
```python
# Simple function
def greet():
    print("Hello!")

# With parameters
def greet(name):
    print(f"Hello, {name}!")

# With return value
def add(a, b):
    return a + b

# Multiple parameters
def compute(x, y, z):
    return x * y + z
```

#### Parameters with Defaults
```python
# Default parameter values
def greet(name, greeting="Hello"):
    print(f"{greeting}, {name}!")

# Multiple defaults
def connect(host="localhost", port=8080, timeout=30):
    pass

# Can call with or without defaults
greet("Alice")              # Uses default greeting
greet("Bob", "Hi")          # Overrides default
connect()                   # All defaults
connect(host="example.com") # Override one default
```

#### Variable Arguments (`*args`, `**kwargs`)
```python
# Variable positional arguments
def sum_all(*args):
    return sum(args)

sum_all(1, 2, 3)           # 6
sum_all(1, 2, 3, 4, 5)     # 15

# Variable keyword arguments
def configure(**kwargs):
    for key, value in kwargs.items():
        print(f"{key} = {value}")

configure(host="localhost", port=8080)

# Mixed parameters
def func(a, b, *args, **kwargs):
    print(f"a={a}, b={b}")
    print(f"args={args}")
    print(f"kwargs={kwargs}")

func(1, 2, 3, 4, x=10, y=20)
# a=1, b=2
# args=(3, 4)
# kwargs={'x': 10, 'y': 20}
```

#### Keyword-Only Arguments
```python
# Parameters after * are keyword-only
def func(a, b, *, c, d):
    return a + b + c + d

# Must call with keywords for c and d
func(1, 2, c=3, d=4)       # OK
# func(1, 2, 3, 4)         # ERROR: c and d must be keyword

# With *args before keyword-only
def func(a, *args, b, c):
    pass

func(1, 2, 3, b=4, c=5)    # a=1, args=(2,3), b=4, c=5
```

#### Type Annotations
```python
# Parameter and return type annotations
def add(a: int, b: int) -> int:
    return a + b

# Complex types
def process(data: list[int]) -> dict[str, int]:
    return {"sum": sum(data), "count": len(data)}

# Optional and Union types
def find(items: list[str], target: str) -> int | None:
    try:
        return items.index(target)
    except ValueError:
        return None

# Generic types
def first(items: list[T]) -> T | None:
    return items[0] if items else None

# Annotations on *args and **kwargs
def func(*args: int, **kwargs: str) -> None:
    pass
```

**Note**: Type annotations are parsed but not yet fully type-checked.

#### Async Functions
```python
# Async function definition
async def fetch_data():
    await client.get("/api/data")
    return data

# Async with parameters
async def download(url: str) -> bytes:
    response = await client.get(url)
    return response.content
```

### Class Definitions

#### Basic Classes
```python
# Simple class
class Point:
    pass

# Class with methods
class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y
    
    def distance(self):
        return (self.x ** 2 + self.y ** 2) ** 0.5

# Class with type annotations
class Person:
    def __init__(self, name: str, age: int):
        self.name: str = name
        self.age: int = age
    
    def greet(self) -> str:
        return f"Hello, I'm {self.name}"
```

#### Inheritance
```python
# Single inheritance
class Animal:
    def speak(self):
        pass

class Dog(Animal):
    def speak(self):
        return "Woof!"

# Multiple inheritance
class Flyable:
    def fly(self):
        pass

class Swimmable:
    def swim(self):
        pass

class Duck(Animal, Flyable, Swimmable):
    def speak(self):
        return "Quack!"
```

#### Class with Keyword Arguments
```python
# Keyword arguments in class definition (for metaclass, etc.)
class MyClass(Base, metaclass=Meta, keyword_arg=value):
    pass

# Multiple keywords
class Config(BaseConfig, frozen=True, slots=True):
    pass
```

### Import Statements

#### Basic Imports
```python
# Import module
import os
import sys
import math

# Import with alias
import numpy as np
import pandas as pd

# Import multiple modules
import os, sys, math
```

#### From Imports
```python
# Import specific names
from os import path, environ
from math import sin, cos, pi

# Import with alias
from collections import OrderedDict as ODict

# Import all (not recommended but supported)
from module import *

# Import multiple with aliases
from pkg import (
    ClassA as A,
    ClassB as B,
    function_c as func_c
)
```

#### Relative Imports
```python
# Relative import from parent
from . import module          # Current package
from .. import module         # Parent package
from ... import module        # Grandparent package

# Relative import with names
from .module import Class
from ..package import function
from ...utils import helper

# Relative with multiple levels
from ....core import Base
```

### Exception Handling

#### Try-Except
```python
# Basic exception handling
try:
    risky_operation()
except Exception:
    handle_error()

# Catch specific exception
try:
    value = int(input())
except ValueError:
    print("Invalid number")

# Multiple except blocks
try:
    result = compute()
except ValueError:
    print("Value error")
except TypeError:
    print("Type error")
except Exception:
    print("Other error")

# Catch and bind exception
try:
    process()
except ValueError as e:
    print(f"Error: {e}")

# Multiple exceptions in one block
try:
    operation()
except (ValueError, TypeError) as e:
    handle_error(e)
```

#### Try-Except-Else
```python
# Else runs if no exception
try:
    result = compute()
except ValueError:
    print("Error")
else:
    print(f"Success: {result}")
```

#### Try-Except-Finally
```python
# Finally always runs (cleanup)
try:
    file = open("data.txt")
    process(file)
except IOError:
    print("Cannot open file")
finally:
    file.close()

# With else and finally
try:
    result = compute()
except ValueError:
    print("Error")
else:
    print("Success")
finally:
    cleanup()
```

#### Raise Statement
```python
# Raise exception
raise ValueError("Invalid value")

# Raise with cause
try:
    operation()
except KeyError as e:
    raise ValueError("Config error") from e

# Re-raise current exception
try:
    process()
except Exception:
    log_error()
    raise    # Re-raises the caught exception
```

#### Assert Statement
```python
# Assert condition
assert condition
assert x > 0
assert len(data) > 0

# Assert with message
assert condition, "Error message"
assert x > 0, "x must be positive"
assert user is not None, "User not found"
```

### Context Managers

#### With Statement
```python
# Basic context manager
with open("file.txt") as f:
    data = f.read()

# Multiple context managers
with open("input.txt") as fin, open("output.txt", "w") as fout:
    data = fin.read()
    fout.write(process(data))

# With optional variable
with lock:
    critical_section()

# Nested context managers
with outer() as o:
    with inner() as i:
        use(o, i)

# Async context managers
async with client.session() as session:
    data = await session.get("/api")
```

### Pattern Matching

#### Match Statement
```python
# Basic match
match value:
    case 0:
        print("zero")
    case 1:
        print("one")
    case _:
        print("other")

# Match with patterns
match point:
    case (0, 0):
        print("origin")
    case (0, y):
        print(f"on y-axis at {y}")
    case (x, 0):
        print(f"on x-axis at {x}")
    case (x, y):
        print(f"point at ({x}, {y})")

# Match with guard
match value:
    case x if x > 0:
        print("positive")
    case x if x < 0:
        print("negative")
    case _:
        print("zero")

# Match with or patterns
match status:
    case "active" | "pending" | "ready":
        process()
    case "error" | "failed":
        handle_error()

# Match with class patterns
match shape:
    case Circle(radius=r):
        area = 3.14 * r ** 2
    case Rectangle(width=w, height=h):
        area = w * h

# Match with sequence patterns
match values:
    case []:
        print("empty")
    case [x]:
        print(f"single: {x}")
    case [x, y]:
        print(f"pair: {x}, {y}")
    case [x, y, *rest]:
        print(f"first: {x}, second: {y}, rest: {rest}")

# Match with mapping patterns
match config:
    case {"host": host, "port": port}:
        connect(host, port)
    case {"path": path, **rest}:
        load_from_path(path, rest)
```

#### Pattern Types
- **Literal patterns**: `case 42:`, `case "hello":`
- **Capture patterns**: `case x:` (bind to variable)
- **Wildcard pattern**: `case _:` (matches anything, doesn't bind)
- **Sequence patterns**: `case [a, b, c]:`
- **Mapping patterns**: `case {"key": value}:`
- **Class patterns**: `case ClassName(arg=val):`
- **Or patterns**: `case pattern1 | pattern2:`
- **As patterns**: `case pattern as name:`

### Other Statements

#### Global Statement
```python
# Declare global variable
x = 10

def modify():
    global x
    x = 20    # Modifies global x

# Multiple globals
def func():
    global a, b, c
    a = 1
    b = 2
    c = 3
```

#### Nonlocal Statement
```python
# Access enclosing scope variable
def outer():
    x = 10
    
    def inner():
        nonlocal x
        x = 20    # Modifies outer's x
    
    inner()
    print(x)  # 20

# Multiple nonlocals
def func():
    a, b = 1, 2
    
    def nested():
        nonlocal a, b
        a = 10
        b = 20
```

#### Del Statement
```python
# Delete variable
x = 10
del x         # x no longer exists

# Delete multiple
del a, b, c

# Delete list elements
del list[0]
del list[1:3]

# Delete dictionary keys
del dict[key]
del dict["name"]

# Delete attributes
del obj.attribute
```

---

## Type Annotations

Type annotations are fully parsed but not yet type-checked.

### Basic Types
```python
# Variable annotations
x: int = 10
name: str = "Alice"
is_active: bool = True
value: float = 3.14

# Without initial value
count: int
message: str
```

### Collection Types
```python
# List with element type
numbers: list[int] = [1, 2, 3]
names: list[str] = ["Alice", "Bob"]

# Dictionary with key and value types
scores: dict[str, int] = {"Alice": 95, "Bob": 87}
config: dict[str, str | int] = {"host": "localhost", "port": 8080}

# Tuple with element types
point: tuple[int, int] = (10, 20)
record: tuple[str, int, float] = ("Alice", 30, 5.5)

# Variable-length tuple
values: tuple[int, ...] = (1, 2, 3, 4, 5)

# Set with element type
unique: set[int] = {1, 2, 3}
```

### Union Types
```python
# Union with | operator (preferred)
value: int | str = 42
result: int | float | None = None

# Optional (equivalent to | None)
name: str | None = None
value: int | None = get_value()

# Multiple types
data: int | float | str | bool = 42
```

### Generic Types
```python
# Generic list
items: list[T] = []

# Generic dictionary
cache: dict[K, V] = {}

# Nested generics
matrix: list[list[int]] = [[1, 2], [3, 4]]
mapping: dict[str, list[int]] = {"a": [1, 2]}
```

### Callable Types
```python
# Function type
callback: Callable[[int, str], bool]
processor: Callable[[list[int]], int]

# No arguments
factory: Callable[[], object]
```

### Special Types
```python
# Any type (accepts anything)
value: Any = 42

# None type
result: None = None
```

### Function Annotations
```python
# Parameter and return types
def add(a: int, b: int) -> int:
    return a + b

# Complex parameter types
def process(data: list[int], config: dict[str, str]) -> bool:
    pass

# Optional return
def find(items: list[str], target: str) -> int | None:
    pass

# No return (returns None)
def log(message: str) -> None:
    print(message)
```

**Note**: All type annotations shown above are parsed correctly, but full type checking and inference is still in progress.

---

## Decorators

Decorators modify or enhance functions and classes.

### Function Decorators
```python
# Simple decorator
@decorator
def function():
    pass

# Decorator with arguments
@decorator(arg1, arg2)
def function():
    pass

# Multiple decorators (applied bottom-to-top)
@decorator1
@decorator2
@decorator3
def function():
    pass

# Decorator with function call
@decorator()
def function():
    pass

# Decorator with attribute access
@module.decorator
def function():
    pass

# Decorator with complex expression
@get_decorator("name", config)
def function():
    pass
```

### Class Decorators
```python
# Simple class decorator
@dataclass
class Point:
    x: int
    y: int

# Decorator with arguments
@register(name="MyClass", version=1)
class MyClass:
    pass

# Multiple decorators
@decorator1
@decorator2
class MyClass:
    pass

# Decorator with complex call
@decorator(setting=True, value=42)
class Config:
    pass
```

### Common Decorator Patterns
```python
# Property decorator
class Circle:
    @property
    def area(self):
        return 3.14 * self.radius ** 2

# Static method
class Utils:
    @staticmethod
    def helper():
        pass

# Class method
class Factory:
    @classmethod
    def create(cls):
        return cls()

# Async decorator
@async_timer
async def fetch_data():
    pass
```

---

## Important Notes

### Implementation Status

1. **Frontend Complete (100%)**:
   - ✅ Lexer: All 69 token types, 115 tests passing
   - ✅ Parser: All statements and expressions, 255 tests passing
   - ✅ AST: 67 node types fully defined
   - ✅ Semantic Analysis: In progress (97 tests passing)

2. **Indentation and Blocks**:
   - ✅ INDENT/DEDENT tokens fully implemented
   - ✅ Python-style significant indentation works correctly
   - ✅ Block-based structure (if, while, for, def, class, etc.) fully supported

3. **Type System**:
   - ✅ Type annotations are fully parsed
   - ⏳ Type checking is in progress (not yet complete)
   - ⏳ Type inference is in progress
   - All type syntax shown in this document is parsed correctly

4. **Not Yet Implemented**:
   - ⏳ Full semantic analysis (in progress - 70% complete)
   - ❌ Code generation
   - ❌ Standard library
   - ❌ Runtime system
   - ❌ Compilation to machine code

### Known Limitations

1. **Type Checking**: Type annotations are parsed but not yet enforced or checked for correctness.

2. **Closure Resolution**: Name resolution for closures has some TODOs that need to be completed.

3. **No Execution**: The compiler can tokenize and parse code, but cannot yet execute it or generate binaries.

4. **Standard Library**: No standard library is available yet. Built-in functions like `print()`, `range()`, `len()`, etc. are parsed but not implemented.

### Test Coverage

This syntax reference is backed by **467 comprehensive tests**:
- **115 lexer tests**: All token types, literals, operators, keywords
- **255 parser tests**: All expressions, statements, and language constructs
- **97 semantic analyzer tests**: Symbol tables, name resolution, scope management

Every syntax feature documented here has corresponding tests that verify correct parsing.

### Future Extensions

As semantic analysis and code generation progress, additional features will be added:
- Full type inference
- Generic functions and classes
- Advanced pattern matching features
- Optimization passes
- Native code generation

---

## Cross-References

For more information about the Silk compiler and language:

- **[ARCHITECTURE.md](ARCHITECTURE.md)**: Detailed explanation of compiler phases and architecture
- **[TODO.md](TODO.md)**: Development roadmap and planned features
- **[CHANGELOG.md](CHANGELOG.md)**: Version history and changes
- **[README.md](../README.md)**: Project overview and getting started guide

---

**Last Updated**: December 9, 2025
**Frontend Version**: 0.1.0 (Phase 1 Complete)
**Test Count**: 467 tests passing (115 lexer + 255 parser + 97 semantic)

---

*This document represents the current state of the Silk language frontend. All syntax shown here is fully parsed and validated. Code generation and runtime support are in active development.*
