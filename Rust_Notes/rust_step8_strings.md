
# 🧩 Step 8: Strings (in Depth)

Rust provides two main types for text:
- `&str` — string slice (borrowed, immutable)
- `String` — growable, heap-allocated string (owned)

---

## 📌 1. `&str`: String Slice

```rust
let s: &str = "hello"; // string literal
```

---

## 📌 2. `String`: Growable Heap String

```rust
let mut s = String::from("Hello");
s.push_str(" world");
```

---

## 🔄 Converting Between `String` and `&str`

| Operation                   | Result       |
|----------------------------|--------------|
| `"text".to_string()`       | `String`     |
| `String::from("text")`     | `String`     |
| `s.as_str()`               | `&str`       |
| `&s` (where `s: String`)   | `&str`       |

---

## 🧵 Indexing: ❌ Not Allowed!

```rust
let s = String::from("hello");
// let h = s[0]; ❌ Error
```

---

## 🔠 Iterating over Characters

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

---

## 🛠️ Common `String` Methods

| Method           | Description                            |
|------------------|----------------------------------------|
| `.len()`         | Length in bytes                        |
| `.is_empty()`    | Checks if string is empty              |
| `.push(char)`    | Adds a character                       |
| `.push_str(&str)`| Appends a string slice                 |
| `.replace()`     | Replaces parts of the string           |
| `.contains()`    | Checks for substring                   |
| `.split_whitespace()` | Splits on whitespace              |
| `.trim()`        | Removes leading/trailing whitespace    |
| `.to_uppercase()`| Returns an uppercase version           |

---

## ✂️ Example

```rust
let s = String::from("  Hello Rust!  ");
println!("{}", s.trim()); // "Hello Rust!"
println!("{}", s.contains("Rust")); // true
println!("{}", s.replace("Rust", "World")); // "  Hello World!  "
```

---

## 📦 String Capacity and Memory

```rust
let mut s = String::with_capacity(10);
println!("{}", s.capacity()); // 10

s.push_str("hello");
println!("{}", s.capacity()); // Still 10 or more
```

---

## 🧪 Quiz

1. What does this print?
   ```rust
   let s = String::from("hello");
   println!("{}", &s[0..2]);
   ```

2. How to convert a `String` to `&str`?

3. What’s the difference between `.chars()` and `.bytes()`?

---

## 🧠 Quiz Answers

1. ❌ This will panic if slicing hits multibyte characters.  
   - For ASCII like `"hello"`, prints `"he"`

2. ✅ Convert using:
   ```rust
   let s = String::from("hello");
   let slice: &str = &s;
   let slice2: &str = s.as_str();
   ```

3. ✅ `.chars()` → Unicode scalar values  
   `.bytes()` → Raw UTF-8 bytes

   ```rust
   let s = "न";
   println!("{}", s.len());           // 3 bytes
   println!("{}", s.chars().count()); // 1 character
   ```
