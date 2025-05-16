
# 🧩 Step 7: Slices

A **slice** is a reference to a **contiguous sequence of elements** in a collection. It does not own the data — it just borrows a part of it.

---

## 🧵 String Slices

```rust
let s = String::from("hello world");

let hello = &s[0..5];  // "hello"
let world = &s[6..11]; // "world"

println!("{} {}", hello, world);
```

### ✅ You can also omit the bounds:
```rust
let part = &s[..5];
let part = &s[6..];
let part = &s[..];
```

---

## 🧱 Array Slices

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..4]; // [2, 3, 4]
println!("{:?}", slice);
```

---

## 🚨 Safety

```rust
let s = String::from("hello");
let slice = &s[0..100]; // ❌ panics at runtime
```

- Rust ensures string slices don't split UTF-8 characters.

---

## 📊 Visual Example (String)

```text
let s = String::from("hello world");

Index:  0 1 2 3 4 5 6 7 8 9 10
Chars:  h e l l o   w o r l d
```

- `&s[0..5]` → `"hello"`
- `&s[6..11]` → `"world"`

---

## 🛠️ Important Concepts & Methods

| Concept        | Description                                       |
|----------------|---------------------------------------------------|
| `&str`         | String slice                                      |
| `&[T]`         | Slice of array or vector                          |
| `[start..end]` | Slice syntax (exclusive of end)                   |
| `.len()`       | Returns length of slice                           |
| `.get(start..end)` | Returns `Option<&[T]>` safely                  |
| `.split_at(n)` | Splits slice into two at index `n`               |

---

## 🧪 Quiz

1. What will this print?
   ```rust
   let s = String::from("abcdef");
   let sub = &s[2..5];
   println!("{}", sub);
   ```

2. Can this slice safely compile?
   ```rust
   let s = String::from("hi");
   let x = &s[0..3];
   ```

3. What's the type of this?
   ```rust
   let a = [10, 20, 30];
   let s = &a[1..];
   ```

---

## 🧠 Quiz Answers

1. ✅ This will print:
   ```
   cde
   ```

2. ❌ This will panic at runtime.
   - `"hi"` is 2 bytes, so `&s[0..3]` is out of bounds.

3. ✅ The type of `s` is:
   ```
   &[i32]
   ```
