# Rust Lesson 01: Basic Concepts — Structured Notes

---

## 🔧 Function: `var_ass_mut`

### 📌 Topics Covered:
- Variable declaration (`let`)
- Type inference
- Shadowing
- Mutability (`mut`)
- Constants (`const`)

### 🧾 Key Points:
- `let` is used to declare variables.
- Variables are **immutable** by default.
- Use `mut` to allow reassignment.
- Shadowing allows reusing variable names with a new type or value.
- Constants must be explicitly typed and are always immutable.

```rust
let x: i32 = 10;
let y = 11;
let x = "a string?"; // shadowed
let mut z = 10;
z = z + 1;
const _TRUE: i32 = 1;
```

---

## 🔧 Function: `vals_types`

### 📌 Topics Covered:
- Base types (integers, floats, booleans, characters)
- Compound types (tuples, arrays)
- Type casting (`as`)
- Array indexing
- Pattern matching
- Safe user input & parsing
- Handling `Result` with `match`

### 🧾 Base Types:
- `i32`, `i64` for integers
- `f32`, `f64` for floats
- `bool` and comparison
- `char` supports Unicode

### 🧾 Compound Types:
- **Tuples**: fixed-size, mixed types
  ```rust
  let tup: (i32, f64, char) = (1, 2.5, 'z');
  let x = tup.0;
  ```
- **Arrays**: fixed-size, same-type
  ```rust
  let a = [3; 5]; // [3, 3, 3, 3, 3]
  ```

---

### 📌 Detailed Explanation: Input Handling in Rust

```rust
let mut input_text = String::new();
io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

let trimmed = input_text.trim();
match trimmed.parse::<u32>() {
    Ok(mut i) => {
        println!("Integer input: {}", i);
        if i > 5 {
            i = 5;
        }
        let _element = a[i as usize];
    },
    Err(..) => println!("this was not an integer: {}", trimmed),
};
```

#### 🔍 Steps:
1. **Read input** into a mutable string.
2. **Trim** whitespace and newline.
3. **Parse** the string into `u32`.
4. **Match** result:
  - `Ok(i)`: Clamp `i` to 5 and safely index the array.
  - `Err`: Print error message.

---

## 🔧 Function: `expressions`

### 📌 Topics Covered:
- Control flow (`if`, `else`)
- Loop types: `loop`, `while`, `for`
- Iteration using `iter`

### 🧾 Loop Types:
- `loop {}`: Infinite unless `break`
- `while cond {}`: Runs while condition is true
- `for val in 1..=n {}`: Iterates using a range

```rust
let mut c = 0;
loop {
    c += 1;
    if c == 4 { break; }
}
while c != 0 {
    c -= 1;
}
for n in 1..=50 {
    // fizzbuzz logic
}
```

### 🧾 Array Iteration:
- `.iter()` borrows each element without consuming the array
```rust
let a = [10, 20, 30];
for element in a.iter() {
    println!("{}", element);
}
```

---

## 🔧 Module: `testing`

### 📌 Topics Covered:
- Unit testing using `#[test]`
- `assert_eq!()` for validation
- Private test module via `#[cfg(test)]`

```rust
#[test]
fn test_crapadd() {
    assert_eq!(crapadd(1, 3), 2);
}
```

---

## 🔧 Module: `testfuns`

### 📌 Topics Covered:
- Function definitions
- Return values via `return` and expressions
- `pub` for visibility across modules

```rust
pub fn crapadd(x: i32, _y: i32) -> i32 {
    return x + x;
}
pub fn okadd(x: i32, y: i32) -> i32 {
    x + y
}
```

---

This layout organizes Rust basics as they appear function-by-function in your lecture.