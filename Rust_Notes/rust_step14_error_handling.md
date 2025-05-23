
# 🧩 Step 14: Error Handling (`Option` and `Result`)

Rust provides safe enums to handle absence of values and errors:
- `Option<T>` for optional values
- `Result<T, E>` for operations that can fail

---

## 🔹 `Option<T>`

```rust
let some_val: Option<i32> = Some(5);
let no_val: Option<i32> = None;
```

### Pattern Matching

```rust
match some_val {
    Some(v) => println!("Got: {}", v),
    None => println!("Nothing"),
}
```

### Shorthand

```rust
if let Some(v) = some_val {
    println!("Quick match: {}", v);
}
```

---

## 🔸 `Result<T, E>`

```rust
use std::fs::File;
let result = File::open("my_file.txt");
```

### Pattern Matching

```rust
match result {
    Ok(file) => println!("Opened file!"),
    Err(e) => println!("Error: {}", e),
}
```

### `unwrap()` and `expect()`

```rust
let file = File::open("file.txt").unwrap();
let file = File::open("file.txt").expect("Failed to open");
```

---

## ✨ Common Methods: `Option<T>`

| Method           | Description                         | Example |
|------------------|-------------------------------------|---------|
| `.is_some()`      | `true` if it's `Some`               | `Some(3).is_some()` → `true` |
| `.unwrap()`       | Returns value or panics             | `Some(3).unwrap()` → `3` |
| `.unwrap_or(x)`   | Fallback if `None`                  | `None.unwrap_or(10)` → `10` |
| `.map(f)`         | Applies function                    | `Some(3).map(|x| x + 1)` → `Some(4)` |
| `.and_then(f)`    | Chains and returns `Option<T>`      | `Some(3).and_then(|x| Some(x * 2))` |
| `.expect(msg)`    | Panics with custom message          | `Some(3).expect("missing")` |

### Code Example

```rust
let opt = Some(5);
println!("{}", opt.is_some());           // true
println!("{}", opt.unwrap());            // 5
println!("{}", opt.unwrap_or(0));        // 5

let new_opt = opt.map(|x| x * 10);       // Some(50)
println!("{:?}", new_opt);

let chained = opt.and_then(|x| Some(x + 3)); // Some(8)
println!("{:?}", chained);
```

---

## ✨ Common Methods: `Result<T, E>`

| Method           | Description                         | Example |
|------------------|-------------------------------------|---------|
| `.is_ok()`        | `true` if `Ok`                     | `Ok(2).is_ok()` |
| `.is_err()`       | `true` if `Err`                    | `Err("e").is_err()` |
| `.unwrap()`       | Returns value or panics            | `Ok(3).unwrap()` |
| `.unwrap_or(x)`   | Fallback value                     | `Err("err").unwrap_or(99)` |
| `.map(f)`         | Applies function to `Ok`           | `Ok(3).map(|x| x + 1)` |
| `.expect(msg)`    | Panic with message if `Err`        | `Ok(3).expect("fail")` |
| `.and_then(f)`    | Chains more `Result` logic         | `Ok(2).and_then(|x| Ok(x * 2))` |

### Code Example

```rust
let res: Result<i32, &str> = Ok(5);
println!("{}", res.is_ok());               // true
println!("{}", res.unwrap());              // 5
println!("{}", res.unwrap_or(0));          // 5

let mapped = res.map(|x| x * 10);          // Ok(50)
println!("{:?}", mapped);

let chained = res.and_then(|x| Ok(x + 3)); // Ok(8)
println!("{:?}", chained);

let err: Result<i32, &str> = Err("fail");
println!("{}", err.unwrap_or(42));         // 42
```

---

## 🧪 Quiz

1. What does this print?

```rust
let x: Option<i32> = None;
println!("{}", x.unwrap_or(99));
```

2. Will this panic?

```rust
let x: Option<i32> = None;
println!("{}", x.unwrap());
```

3. What's the type of this?

```rust
let x = "42".parse::<i32>();
```

---

## 🧠 Quiz Answers

1. ✅ `99`
2. ❌ Yes, this panics because `unwrap()` called on `None`
3. ✅ `Result<i32, std::num::ParseIntError>`
