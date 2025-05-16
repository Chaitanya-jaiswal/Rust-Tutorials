
# 🧩 Step 10: HashMaps

A `HashMap<K, V>` stores key-value pairs.

---

## 🧰 Creating a HashMap

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

---

## 🔍 Accessing Values

```rust
let score = scores.get("Blue");
```

---

## 🔁 Iterating

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

---

## 🧹 Overwriting & Updating

```rust
scores.insert(String::from("Blue"), 20);
scores.entry(String::from("Green")).or_insert(30);
```

---

## ➕ Modify a Value Based on Key

```rust
let text = "hello world hello";
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

---

## 🛠️ Common Methods

| Method             | Description                             |
|--------------------|-----------------------------------------|
| `.insert(k, v)`    | Insert or overwrite                     |
| `.get(&k)`         | Retrieve value (Option<&V>)             |
| `.entry(k)`        | Conditional insert                      |
| `.remove(&k)`      | Removes entry                           |
| `.contains_key(&k)`| Checks if key exists                    |
| `.keys()`          | Iterator over keys                      |
| `.values()`        | Iterator over values                    |

---

## 📛 Ownership Notes

- `.insert()` moves key and value unless cloned.

---

## 🧪 Quiz

1. What will this print?
   ```rust
   let mut map = HashMap::new();
   map.insert("a", 1);
   map.insert("a", 2);
   println!("{:?}", map.get("a"));
   ```

2. Will this compile?
   ```rust
   let mut map = HashMap::new();
   let key = String::from("name");
   map.insert(key, String::from("Alice"));
   println!("{}", key);
   ```

3. How do you increment a count in a `HashMap`?

---

## 🧠 Quiz Answers

1. ✅ This will print:
   ```
   Some(2)
   ```

2. ❌ This will not compile.
   - Ownership of `key` was moved into the map.

3. ✅ Use `.entry()` and `or_insert()`:

   ```rust
   let mut map = HashMap::new();
   let word = "hello";
   let count = map.entry(word).or_insert(0);
   *count += 1;
   ```
