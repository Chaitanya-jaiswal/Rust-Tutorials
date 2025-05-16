
# 🧩 Step 5: Ownership

Ownership is the core concept that enables Rust to manage memory without a garbage collector, ensuring safety and performance at compile time.

---

## 🔑 What is Ownership?

> In Rust, each value has a single owner, and when the owner goes out of scope, the value is automatically deallocated.

### 📜 Ownership Rules

1. Each value in Rust has a **single owner**.
2. When the owner goes out of scope, the value is **dropped**.
3. A value can be **moved** to another variable (ownership transferred).
4. There can be **only one owner** at a time (unless borrowed).

---

## 📦 Example: Basic Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2
    // println!("{}", s1); // ❌ Compile Error: s1 is no longer valid
}
```

---

## 🧫 Stack vs Heap

| Stack                         | Heap                             |
|------------------------------|----------------------------------|
| Fixed-size, fast allocation  | Dynamic size, slower allocation  |
| `i32`, `bool`, `char`, etc.  | `String`, `Vec`, `Box`, etc.     |
| Automatically copied         | Ownership rules apply            |

---

## ✅ Types with `Copy` Trait

```rust
let x = 5;
let y = x;
println!("x: {}, y: {}", x, y); // ✅ Both are valid
```

---

## 🧪 Cloning

```rust
let s1 = String::from("hi");
let s2 = s1.clone(); // Deep copy
println!("s1: {}, s2: {}", s1, s2);
```

---

## 🛠️ Important Methods and Concepts

| Concept        | Use/Behavior                                  |
|----------------|-----------------------------------------------|
| `.clone()`     | Creates a deep copy (new heap allocation)     |
| `Copy` Trait   | Simple types are duplicated, not moved        |
| `Drop` Trait   | Automatically called when variable goes out of scope |
| Ownership Move | Occurs when assigning or passing non-Copy values |
| Shadowing      | Replaces variable with a new one              |

---

## 👨‍🔬 Function and Ownership

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s is moved
    // println!("{}", s); // ❌ Error

    let x = 5;
    makes_copy(x); // x is still valid ✅
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}
```

---

## 🧪 Quiz

1. What happens here?
   ```rust
   let s1 = String::from("Rust");
   let s2 = s1;
   println!("{}", s1);
   ```

2. Which types are copied instead of moved?

3. What will this print?
   ```rust
   let x = 10;
   let y = x;
   println!("x: {}, y: {}", x, y);
   ```

---

## 🧠 Quiz Answers

1. ❌ This will **not compile**.  
   - `s1` is **moved** to `s2`, so `s1` is no longer valid.  
   - Attempting to use `s1` results in:  
     ```
     error[E0382]: borrow of moved value: `s1`
     ```

2. ✅ Types that are **copied** (not moved) include:
   - `i32`, `u32`, `f64`, `bool`, `char`, `usize`, etc.
   - These implement the `Copy` trait.
   - No heap allocation is involved.

3. ✅ This will compile and print:
   ```
   x: 10, y: 10
   ```
   - Because `x` is of type `i32`, it implements the `Copy` trait.
   - `y` gets a **copy**, not ownership.
