
# 🧩 Step 1: Variables and Mutability

Rust emphasizes **safety and predictability** by making variables immutable by default. Understanding how variables work in Rust — including **mutability**, **shadowing**, and **constants** — is essential.

---

## 1️⃣ Declaring Variables

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
}
```

- `let x = 5;` defines a **new variable** named `x`.
- By default, **variables in Rust are immutable**.
- Attempting to change `x` after declaration will result in a **compiler error**.

---

## 2️⃣ Making Variables Mutable

```rust
fn main() {
    let mut x = 5;
    println!("x is: {}", x);
    x = 6;
    println!("x changed to: {}", x);
}
```

- `mut` keyword makes `x` **mutable**, so its value can be changed.
- This is helpful for counters, accumulators, and buffers.

✅ **Best practice:** use immutability unless mutability is needed.

---

## 3️⃣ Shadowing

Shadowing allows reusing a variable name **by redeclaring it** with `let`.

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is: {}", x); // x is 12
}
```

- Each `let x =` **creates a new variable** in a new scope.
- You can **change the type** when shadowing:

```rust
let spaces = "   ";
let spaces = spaces.len(); // from &str to usize
```

💡 **Key difference vs. `mut`:**
- `mut` allows changing the value **in place**.
- Shadowing replaces the **entire variable** (can even change type).

---

## 4️⃣ Constants

```rust
const MAX_POINTS: u32 = 100_000;
```

- Declared with `const`
- Must have an **explicit type**
- Must be a **compile-time constant**
- **Cannot** be mutable or shadowed
- Naming convention: **ALL_CAPS_WITH_UNDERSCORES**

---

## 5️⃣ Type Inference

Rust has powerful **type inference**, so you often don’t need to specify types:

```rust
let a = 10;      // inferred as i32
let b = 2.5;     // inferred as f64
let c = "Rust";  // inferred as &str
```

You can **explicitly annotate** types:

```rust
let a: u32 = 10;
```

---

## 🧪 Summary Table

| Feature     | Mutable? | Can Shadow | Can Change Type | Type Required |
|-------------|----------|------------|------------------|----------------|
| `let`       | ❌        | ✅         | ✅               | ❌ (optional)  |
| `let mut`   | ✅        | ✅         | ✅ (via shadow)  | ❌ (optional)  |
| `const`     | ❌        | ❌         | ❌               | ✅ (required)  |

---

## 🛠️ Important Methods & Utilities (for Step 1)

| Functionality       | Method/Concept               | Description                                              |
|---------------------|------------------------------|----------------------------------------------------------|
| Convert to string   | `to_string()`                | Turns number or value into a `String`                   |
| Parse from string   | `"123".parse::<i32>()`       | Converts a string to an integer (must unwrap `Result`)  |
| Get memory size     | `std::mem::size_of_val(&x)`  | Returns size (in bytes) of a value                      |
| Underscore for nums | `let x = 1_000_000;`         | Makes large numbers more readable                       |
| Constants           | `std::f64::consts::PI`       | Access math constants like `PI`, `E`, etc.             |

*Debugging methods like `dbg!()` will be covered later.*

---

### ✅ Recap

- Rust promotes immutability by default for safety and concurrency.
- Use `mut` for changing values, and `let` + shadowing for transformations.
- Use `const` for fixed values known at compile time.
- Use `.to_string()` and `.parse::<T>()` for converting between types.
