# Rust

## Tools

- **Rustup** manage rust installations.
- **Cargo** compile manager, package management, general.
- **Rustc** compiler.
- **Rustdoc** documentation builder.

## Commands

- `rustup update` updates cargo, rustc, rustdoc.
- `cargo new [name]` create [name] package.
- `cargo run` runs the current project.
- `cargo clean` cleans build generated files.
- `cargo test` runs tests.

## Files

- **cargo.toml** project metadata.

## Data Types and Structures

### Primitives

#### Signed integers

- i8
- i16
- i32
- i64
- i128
- isize (long 32 - 64)

```rust
let n: i32 = -4;
let m = -120i8;
let h: i8 = 0xcafeu32;
let b: i8 = 0b0010_1010;
let o: i64 = 0o106;
```

#### Unsigned integers

- u8 (byte)
- u16
- u32
- u64
- u128
- usize (long 32 - 64)

```rust
let n: u32 = 8;
let m = 255u8;
let price: u32 = 4_294_967_295;
let c: u8 = b'A';
```

**byte literals escape sequence** b'\'', b'\\', b'\n', b'\r', b'\t'.

#### IEEE floating-point numbers (single and double precision)

- f32
- f64

```rust
let f: f32 = 1.61;
let f2: f64 = 31415.926e-4
```

#### Boolean

- bool

```rust
let b: bool = true;
```

#### Strings

- String (UTF-8 string, dynamically sized)
- char (Unicode character, 32 bits wide **not** i32 or u32)

```rust
let s: String = "Hello world";
let r: String = r#"
Hello world
"#; // raw string
let c: char = "*";
let c2: char = '\xHH';
let c3: char = U+0000;

s.len();
s.chars().count();
```

### Tuples

(...types)

```rust
let t: () = ();
let t2: (i32, bool, char) = (32, false, "h");
t2.0;
t2.1;
t2.2;
let (n, b, c) = t2;
```

### Vectors

Vec<type>

```rust
let v: Vec<f64> = vec![0.367, 2.718, 7.38];
let v2: Vec<i8> = (0..10).collect();
let mut v3: Vec<usize> = Vec::with_capacity(8);
let a = v[0];
let l = v.len();

v3.push(1);
v3.push(2);
v3.len();
v3.capacity();
```

### Arrays

[type; length]

```rust
let a: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
let b = a[0];
let l = b.len()
```

### Slices

### Structs

{}, ()

```rust
struct  S { x: f32, y: f32 };
let s = S { x: 120.0, y: 20.9 };

struct T (i32, char);
let t = T(120, "x");

struct E;
let e = E;
```

### Enums

{}

```rust
```

```rust
enum Attend {
  OnTime,
  Late(u32)
}
```

### Boxes

### References (pointers)

- &table
- &mut e
- *table

### Option

### Result

### Functions

### Closures

### Copy types

Integers, floating-point, char, bool, array of copy types, tuple of copy types.

### Reference count and Atomic reference count

- Rc<T>
- Arc<T>

## Arithmetic

- + - * / %
- += -= *= /= %=
- checked operations (methods) return Option, Some(v) or None
- wrapping operations (methods) return v % n^bits
- saturating operations (methods) return closest mathematically correct result (min - max)

**Operations**

- *_add
- *_sub
- *_mul
- *_div
- *_rem (remainder)
- *_neg (negation)
- *_abs
- *_pow
- *_shl (bitwise left shift)
- *_shr (bitwise right shift)

## Comparison Operators

- ==, !=, <, <=, >, >=.

## Logical Operators

- &&, ||

## Casting

as

```rust
34_i32 as i64
```

## Syntax

### Comments

// inline comments
/* */ multi-line comments

```rust
// some comments
/*
  some multi-line
  comments
*/
```

### Documentation

///

```rust
/// The following function do this
```

### Variables

- names can include _, letters, and numbers.
- names can only start with _ or letters.
- names are case-sensitive.
- variables can not be reassigned unless `mut` is used.

```rust
let a: u64 = 5;
let mut b = a;
b = 35;
```

### Constants

- names can include _, letters, and numbers.
- names can only start with _ or letters.
- names are all uppercase.
- types should always be defined.
- can be created in any scope.
- can only be assigned a constant expression and not something that is calculated in runtime.

```rust
const PORT:u16 = 8000;
```

### Conditionals

```rust
if n < m {}
```

### Loops

```rust
while m != 0 {}
```

```rust
for n in &numbers[1..] {}
```

```rust
for i in 0..n {}
```

#### Infinite loops

```rust
loop {}
```

### Functions

```rust
fn sum(mut n: u64, mut m: u64) -> u64 {
  n + m
}

sum(1,2)
```

### Mutations

```rust
fn gcd(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }
  n
}
```

### Modules

#### Installation

Simple define crates, version, and additional options in `Cargo.toml` file. When running `cargo run` it will fetch and build the required crates.

#### Require

```rust
use std::env;
```

## Attributes

Mark functions and declarations with extra information.
Controls compiler warnings, code style checks, conditional code inclusion, embedded languages, and more.

## Testing

```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3 * 11)
}
```

## Macros

- `assert!` checks that the argument is true otherwise terminates the program (panic).
- `debug_assert!` skip assertions when the program is compiled.
- `assert_eq!` asserts that two expressions are equal to each other.
- `println!` takes a template string, substitutes formatted arguments, and writes the result to the standard output stream.
- `eprintln!` write out error messages to the standard error output stream.
- `format!` takes a template string, substitutes formatted arguments, and returns a new string.

## Terminology

- **panic** abrupt termination of a program with a message including the source location of the failing check.

## Other

- return statement is not required if a function body ends with an expression that is not followed by a semicolon.
- return are usually used for early returns.
- test are ignored in normal compilation.
- trait is a collection of methods that types can implement.
- rust doesn't include exceptions. All errors are handled using either Result or Panic.
- & borrows a reference (borrowing).
- * dereferences a reference, and yields the value it refers to.
- if main does not returns at all, rust assume the program finished successfully (exit code: 0).
- libraries are called crates.
- Rust has type inference.
- Rust supports generics.
- Methods calls take precedence over unary operators. -4.abs() vs (-4).abs().
- Overflows in debug mode panic in prod wraps around.
- Rust consistently permits an extra tailing comma everywhere commas are used.
- Vectors and Arrays can call slice methods.