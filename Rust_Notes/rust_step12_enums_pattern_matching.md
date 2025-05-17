
# 🧩 Step 12: Enums & Pattern Matching

Enums allow you to define a type with multiple possible values (variants).

---

## 📦 Defining an Enum

```rust
enum Direction {
    North,
    South,
    East,
    West,
}
```

---

## 🔄 Using Enums

```rust
let dir = Direction::North;
match dir {
    Direction::North => println!("Going north"),
    Direction::South => println!("Going south"),
    _ => println!("East or West"),
}
```

---

## 📥 Enums with Data

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

---

## 🧠 Pattern Matching

```rust
let msg = Message::Move { x: 10, y: 20 };
match msg {
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    Message::Quit => println!("Quit"),
    _ => println!("Something else"),
}
```

---

## ✅ `Option<T>` Enum (built-in)

```rust
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;
```

---

## 🔍 `if let` and `while let`

```rust
let config = Some(3);
if let Some(n) = config {
    println!("Got {}", n);
}

let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

## 🛠️ Important Concepts

| Feature         | Description                                      |
|-----------------|--------------------------------------------------|
| `enum`          | Declares a type with multiple possible values    |
| `match`         | Exhaustively handles enum variants               |
| `Option<T>`     | Built-in enum: `Some(T)` or `None`               |
| `if let`        | Shorthand for matching one case                  |
| `while let`     | Repeated matching for loops                      |

---

## 🧪 Quiz

1. What will this print?
   ```rust
   enum Light {
       Red, Green
   }
   let l = Light::Green;
   match l {
       Light::Red => println!("Stop"),
       Light::Green => println!("Go"),
   }
   ```

2. Will this compile?
   ```rust
   enum State {
       Code(i32),
       Text(String),
   }

   let s = State::Text(String::from("ok"));
   if let State::Code(val) = s {
       println!("{}", val);
   }
   ```

3. What's the value of `n` here?
   ```rust
   let maybe = Some(42);
   let n = match maybe {
       Some(val) => val,
       None => 0,
   };
   ```

---

## 🧠 Quiz Answers

1. ✅ Output:
   ```
   Go
   ```

2. ❌ Will not compile.
   - Pattern does not match, and ownership of `s` is moved.

3. ✅ `n = 42`
