
# 🧩 Step 13: Modules & Visibility

Rust’s module system helps organize code into namespaces and control visibility.

---

## 📦 1. Declaring a Module (same file)

```rust
mod greetings {
    pub fn say_hello() {
        println!("Hello!");
    }
}

fn main() {
    greetings::say_hello();
}
```

---

## 📁 2. File-based Modules

```
src/
├── main.rs
├── greet.rs
```

### main.rs

```rust
mod greet;

fn main() {
    greet::say_hello();
}
```

### greet.rs

```rust
pub fn say_hello() {
    println!("Hello from file!");
}
```

---

## 🗂️ 3. Nested Modules

```
src/
├── main.rs
├── utils/
│   ├── mod.rs
│   └── greet.rs
```

### main.rs

```rust
mod utils;

fn main() {
    utils::greet::say_hello();
}
```

### utils/mod.rs

```rust
pub mod greet;
```

### utils/greet.rs

```rust
pub fn say_hello() {
    println!("Nested module!");
}
```

---

## 🔒 4. Visibility Rules

| Keyword        | Description                                 |
|----------------|---------------------------------------------|
| `pub`          | Makes item visible outside the module       |
| *Default*      | Items are private by default                |
| `pub(crate)`   | Visible only within the current crate       |

---

## 🛠️ Important Concepts

| Concept               | Description                                 |
|------------------------|---------------------------------------------|
| `mod name`             | Declares a module                           |
| `pub`                  | Makes functions/structs/enums visible       |
| `mod.rs`               | Entry point for nested module               |
| `use`                  | Simplifies access to module items           |

---

## 🧪 Quiz

1. What’s wrong with this code?

```rust
mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(2, 3));
}
```

2. What’s the fix for this?

3. How do you create a nested module with its own file?

---

## 🧠 Quiz Answers

1. ❌ The `add` function is **private**. `main` can’t access it.

2. ✅ Add `pub` to the function:
```rust
pub fn add(a: i32, b: i32) -> i32
```

3. ✅ Create `mod.rs` in a folder and place submodules (e.g. `greet.rs`) inside it. Then declare `pub mod greet;` inside `mod.rs`.
