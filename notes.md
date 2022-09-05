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

- access values using dot notation.
- Values to the left of . are automatically dereferenced.

```rust
let t: () = ();
let t2: (i32, bool, char) = (32, false, "h");
t2.0;
t2.1;
t2.2;
let (n, b, c) = t2;
```

### Collections

**Iterable types:** Collections, HashMaps, Arrays, Vectors, Slices, Ranges, Strings.

#### Vectors

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

#### Arrays

[type; length]

```rust
let a: [f64; 4] = [1.0, 0.0, 0.0, 1.0];
let b = a[0];
let l = b.len()
```

#### Slices

- portion of borrowed slices, arrays, or vectors.
- de portion is determined by a range.


```rust
let v = vec![1,2,3,4,];
let s = &v[0..=2];
```

#### Ranges

**Ranges:**
  - .. full
  - a .. from a
  - .. b to b (non-inclusive)
  - ..= b to b (inclusive)
  - a .. b a to b (non-inclusive)
  - a ..=  b a to b (inclusive)

```rust
std::ops::Range { start: 0, end: 20 }
let r1 = 0..9
```

### Structs

{}, ()

- access values using dot notation.
- Values to the left of . are automatically dereferenced.

```rust
struct  S { x: f32, y: f32 };
let s = S { x: 120.0, y: 20.9 };

let sx = s.x

struct T (i32, char);
let t = T(120, "x");

let t1 = t.1

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
- += -= *= /= %= (compound assignment)
- & | ^ << >> ! (bitwise operators)
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

## Logical Operators (short-circuiting)

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

### Blocks

```rust
let msg = {;
  let dandelion_control = puffball.open();

  dandelion_control.release_all_seeds(launch_codes);

  dandelion_control.get_status()
};
```

### Conditionals

- Conditions must be an expression of type bool.
- Rust does not implicitly convert to boolean values.
- else if and else are optional.
- no else behave as an empty block else `{}`.
- All block of an if expression and match expressions must produce values of the same type.

```rust
if n < m {
} else if n > m {
} else {
}
```

- Patterns can match a range of values.
- Unpack tuples
- Match against individual fields of structs.
- Chase references.
- Borrow parts of a value.
- Rust prohibits match expressions that do not cover all possible values.
- ? is shorthand for Result type match

```rust
match code {
  0 => println!("OK"),
  1 => println!("Wires Tangled"),
  2 => println!("User Asleep"),
  _ => println!("Unrecognized Error {}", code)
}
```

```rust
let output = File::create(filename)?;

// return from the enclosing function
let output = match File::create(filename) {
  Ok(f) => f,
  Err(err) => return Err(err),
};
```

- If let can be used when we need to get data out of Option or Result
- Alternative to match.
- Is an expression shorthand for a match with just one pattern.

```rust
if let Some(cookie) = request.session_cookie {
  return restore_session(cookie);
}

if let Err(err) = show_cheesy_anti_robot_task() {
  log_robot_attempt(err);
  politely_accuse_user_of_being_a_robot();
} else {
  session.mark_as_human();
}
```

### Loops

- While and for always produce ().
- Loop can produce a value if specified.
- Conditions must be of type bool.
- Break keyword only works on loops.

```rust
while m != 0 {}
```

- The expression should match the pattern in order to run.
- Analogous to if let expression.

```rust
while let pattern = expression {
}
```

- Infinite loops until `break` or `return` or thread panics.
- We can give break an expression, whose value becomes that of the loop.
- All break values within a loop must be the same type.

```rust
loop {
  break "hello";
}
```

- Evaluates the iterable on every run.

```rust
for i in 0..n {}
```

- Consumes the value iterable (move reference).
- If we don't want to consume the value then we should iterate over a reference.
- Iterating over a mut reference provides a mut reference to each element.

```rust
for n in &numbers[1..] {}
```

- Loops can be label with a lifetime.
- Break can have both a label and a value expression.
- Labels can also be used with continue.

```rust
let sqrt = 'outer: loop {
  let n = next_number();

  for i in 1.. {
    let square = i * i;

    if square == n {
      break 'outer i;
    }
    if square > n {
      break;
    }
  }
};
```

### Functions

```rust
fn sum(mut n: u64, mut m: u64) -> u64 {
  n + m
}

sum(1,2)
```

### Return

- exits the current function, returning a value to the caller.
- return without value is shorthand for `return ();`
- return statement is not required if a function body ends with an expression that is not followed by a semicolon.
- return are usually used for early returns.

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

### User defined Types

- can call methods with . operator.
- methods can be called for values, references, or smart pointers.
- . operator automatically dereferences or borrow a reference as needed.
- methods can be chained.

```rust
server
  .bind("127.0.0.1:3000")
  .expect("error binding server to address")
  .run()
  .expect("error running server");
```

- :: is used to call type-associated functions.
- similar to static methods in OOP.

```rust
let v = Vec::new();
```

- turbofish operator allow us to define generics in function and method calls.

```rust
let v = Vec::<i32>::with_capacity(10);

let v = (0..10).collect::<Vec<i32>>();
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

## Ownership, Borrowing, and lifetime

- Who is the owner of a value. In Rust a value (no copy type) can only be owned by a single variable.
- When we assign a variable to another one we are actually moving the ownership and the first one cease to exist.
- In rust you can not assign uninitialized variables. If we try to do something with a variable that no longer owns a value then Rust panics.
- Move ownership applies for assignments, function calls, function returns, for in loops.
- If we don't want to move the ownership of a value we then borrow that value with a reference `&`.
- Assigning a reference makes its referent read-only. We can not move its ownership, preventing dangling references.
- Lifetimes are like scope but for references. A referrer should never outlive the referred.

```rust
fn create_vector() {
  let mut a: Vec<&i32> = vec![];
  let mut b = a;

  // a - uninitialized

  {
    let n: i32 = 8;
    b.push(&n); // panic  - b outlives n
  }
}
```

- Whenever a reference type appears inside another type's definition, we must write out its lifetime.

```rust
struct S<'a> {
  n: &'a i32
}

struct D<'b> {
  s: S<'b>
}
```

- Rust doesn't compile if the code is not safe. Try without lifetimes and the add them as needed.
- Multiple read-only references can be shared at a time, but we can share only one mutable.

## Terminology

- **panic** abrupt termination of a program with a message including the source location of the failing check.

## Keywords

fn, return, let, const, mut, struct, use, if / else if / else, match, for in, while, loop, break, continue

## Other

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
- Everything is an expression, everything produces a value.
- It's consider a good practice to omit the types whenever they can be inferred.
