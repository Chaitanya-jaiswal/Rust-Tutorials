
# 🧩 Step 6: References & Borrowing

Rust allows you to refer to values without taking ownership using *references*. This is called **borrowing**, and it lets multiple parts of code access the same data safely.

---

## 🔗 What is a Reference?

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
println!("Length: {}", len);
```

---

## 📥 Function with References

```rust
fn calculate_length(s: &String) -> usize {
    s.len()
}
```

---

## ✏️ Mutable References

```rust
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

let mut s = String::from("hello");
change(&mut s);
println!("{}", s); // "hello, world"
```

---

## 🚫 Rules of Borrowing

- You can have **either**:
  - multiple immutable references (`&T`), **or**
  - one mutable reference (`&mut T`), **not both**.

```rust
let r1 = &s;
let r2 = &s; // ✅ multiple immutable refs
```

```rust
let r1 = &mut s;
let r2 = &mut s; // ❌ only one mutable ref at a time
```

```rust
let r1 = &s;
let r2 = &mut s; // ❌ cannot mix & and &mut
```

---

## 🚨 Dangling References

```rust
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // ❌ invalid: s will be dropped
// }
```

✔️ Instead:

```rust
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
```

---

## 🛠️ Important Concepts & Methods

| Concept              | Description                                      |
|----------------------|--------------------------------------------------|
| `&T`                 | Immutable reference                              |
| `&mut T`             | Mutable reference                                |
| Borrowing            | Passing references instead of moving             |
| `.len()`             | Returns length of collections                    |
| `.push_str()`        | Adds to `String`, requires `&mut String`         |
| Reference safety     | Checked at compile-time                          |
| Dangling prevention  | Lifetime rules prevent invalid references        |

---

## 🧪 Quiz

1. What will this print?
   ```rust
   let s = String::from("Rust");
   let len = get_len(&s);
   println!("{} has length {}", s, len);

   fn get_len(s: &String) -> usize {
       s.len()
   }
   ```

2. Can this compile?
   ```rust
   let mut s = String::from("test");
   let r1 = &mut s;
   let r2 = &mut s;
   println!("{}, {}", r1, r2);
   ```

3. Will this compile?
   ```rust
   let mut s = String::from("data");
   let r1 = &s;
   let r2 = &mut s;
   println!("{}, {}", r1, r2);
   ```

---

## 🧠 Quiz Answers

1. ✅ This will compile and print:
   ```
   Rust has length 4
   ```

2. ❌ This will not compile.  
   - Cannot have two mutable references at the same time.

3. ❌ This will not compile.  
   - Cannot mix mutable and immutable references at the same time.
