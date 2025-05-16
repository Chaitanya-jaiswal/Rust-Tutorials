
# 🧩 Step 2: Data Types

Rust is a **statically and strongly typed** language — types are known at compile time, and conversions between types must be **explicit**.

---

## 📚 Data Type Categories

### 1. Scalar Types

| Type     | Description                     | Example           |
|----------|---------------------------------|-------------------|
| `i8`-`i128`, `isize` | Signed integers        | `let a: i32 = -5;` |
| `u8`-`u128`, `usize` | Unsigned integers      | `let b: u8 = 255;` |
| `f32`, `f64`         | Floating-point numbers | `let c: f64 = 3.14;` |
| `bool`              | Boolean values         | `let d: bool = true;` |
| `char`              | Unicode scalar values  | `let e: char = '♥';` |

Default types:
- Integer literals → `i32`
- Float literals → `f64`

---

### 2. Compound Types

#### 🧊 Tuples

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
println!("y = {}", y);
println!("First: {}", tup.0);
```

- Group multiple values of different types.
- Fixed size.
- Access by destructuring or indexing.

#### 🧱 Arrays

```rust
let arr = [1, 2, 3, 4, 5];
let first = arr[0];
```

- All elements must be of the same type.
- Fixed length.
- Stored on the stack.

```rust
let arr = [0; 5]; // [0, 0, 0, 0, 0]
```

---

### ⚙️ Integer Overflow

- Panics in debug mode.
- Wraps around in release mode.

```rust
let x: u8 = 255;
let y = x + 1; // Panics or becomes 0
```

---

### 🔁 Type Casting

```rust
let x = 5;
let y = x as f64 + 3.2;
```

---

## 🛠️ Important Methods and Constants

### 📏 Numeric Methods

```rust
let n: i32 = -10;
n.abs();       // 10
n.signum();    // -1
n.pow(2);      // 100
```

### 📐 Float Methods

```rust
let f = -3.5f32;
f.floor(); // -4.0
f.ceil();  // -3.0
f.round(); // -4.0
f.sqrt();  // Error for negative numbers
```

---

## 🧮 Float Methods Explained

### 🔻 `.floor()`
Returns the largest integer ≤ value.

```rust
let x = 3.7_f64;
x.floor(); // 3.0
```

### 🔺 `.ceil()`
Returns the smallest integer ≥ value.

```rust
let x = 3.2_f64;
x.ceil(); // 4.0
```

### 🟰 `.round()`
Rounds to the nearest integer (ties round to even).

```rust
let x = 2.5_f64;
x.round(); // 2.0

let y = 3.5_f64;
y.round(); // 4.0
```

### ❓ `.is_nan()`
Check for NaN (Not a Number).

```rust
let x = 0.0 / 0.0;
x.is_nan(); // true
```

### ♾️ `.is_infinite()`
Check for infinity.

```rust
let x = 1.0 / 0.0;
x.is_infinite(); // true
```

### ➖ `.is_sign_negative()`
Check if number is negative, including -0.0.

```rust
let x = -0.0_f64;
x.is_sign_negative(); // true
```

---

## ✅ Best Practices

- Use `usize` for indexing.
- Prefer `i32`, `f64` unless you need special size/precision.
- Use tuple destructuring for clarity.
- Use `[val; len]` for array initialization.

---

## ❓ Quiz

1. Difference between `usize` and `u32`?
2. What does a `u8` overflow become in release mode?
3. How to access 3rd item in `(1, 2, 3)`?

---

