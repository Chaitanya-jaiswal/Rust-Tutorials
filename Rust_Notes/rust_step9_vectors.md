
# 🧩 Step 9: Vectors (`Vec<T>`)

A **vector** in Rust is a growable array. It's the go-to container when you need to store a variable number of items of the same type on the heap.

---

## 🔧 Creating Vectors

```rust
let v: Vec<i32> = Vec::new();
let mut v = vec![1, 2, 3];
```

---

## ➕ Adding Elements

```rust
let mut v = Vec::new();
v.push(10);
v.push(20);
```

---

## ➖ Removing Elements

```rust
let last = v.pop(); // Option<T>
```

---

## 🎯 Accessing Elements

```rust
let third = &v[2];
let maybe_third = v.get(2);
```

---

## 🔁 Iterating Over Vectors

```rust
for x in &v {
    println!("{}", x);
}

for x in &mut v {
    *x += 1;
}
```

---

## 🧪 Vector with Mixed Behavior

```rust
let v = vec![String::from("a"), String::from("b")];

let first = &v[0];
// v.push(String::from("c")); ❌ compile error
println!("First: {}", first);
```

---

## 🛠️ Common Methods

| Method          | Description                             |
|------------------|-----------------------------------------|
| `.push(val)`     | Adds element to end                     |
| `.pop()`         | Removes and returns last element        |
| `.get(i)`        | Safe access (returns `Option`)          |
| `.len()`         | Current number of elements              |
| `.is_empty()`    | Checks if empty                         |
| `.contains(&val)`| Checks if element exists                |
| `.clear()`       | Removes all elements                    |
| `.remove(i)`     | Removes element at index `i`            |
| `.insert(i, val)`| Inserts at index                        |

---

## 🧪 Quiz

1. What does this print?
   ```rust
   let mut v = vec![10, 20, 30];
   v.push(40);
   println!("{}", v[2]);
   ```

2. Can this compile?
   ```rust
   let mut v = vec![1, 2, 3];
   let first = &v[0];
   v.push(4);
   println!("{}", first);
   ```

3. What does `.get(i)` return if index is out of bounds?

---

## 🧠 Quiz Answers

1. ✅ This will print:
   ```
   30
   ```

2. ❌ This will not compile.
   - Cannot mutate while immutable reference is active.

3. ✅ `.get(i)` returns:
   ```
   Option<&T>
   ```
   - Returns `None` if out of bounds.
