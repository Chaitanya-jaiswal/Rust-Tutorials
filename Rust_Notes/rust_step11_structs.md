
# 🧩 Step 11: Structs

Structs are custom data types that let you group related data.

---

## 📦 Defining a Struct

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

---

## 🛠 Creating Struct Instances

```rust
let user1 = User {
    email: String::from("user@example.com"),
    username: String::from("user123"),
    active: true,
    sign_in_count: 1,
};
```

---

## 🧾 Field Access & Mutability

```rust
println!("{}", user1.email);
let mut user2 = user1;
user2.email = String::from("new@example.com");
```

---

## 📥 Struct Update Syntax

```rust
let user3 = User {
    email: String::from("copy@example.com"),
    ..user2
};
```

---

## ⚡ Tuple Structs

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

---

## 🪪 Unit-like Structs

```rust
struct Marker;
let _m = Marker;
```

---

## 📌 Associated Functions

```rust
impl User {
    fn sign_in_status(&self) -> String {
        if self.active {
            String::from("Active")
        } else {
            String::from("Inactive")
        }
    }

    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}
```

---

## 🛠️ Important Concepts

| Feature                | Description                             |
|------------------------|-----------------------------------------|
| `struct`               | Declares a custom data type             |
| `mut`                  | Must be applied to the whole instance   |
| `..source`             | Update syntax to reuse fields           |
| `impl`                 | Implementation block for methods        |
| `self` / `&self`       | Reference to the instance               |
| `Tuple structs`        | Structs with unnamed fields             |
| `Unit-like structs`    | Empty structs (used for traits/tags)    |

---

## 🧪 Quiz

1. What does this print?
   ```rust
   struct Point(i32, i32);
   let p = Point(5, 10);
   println!("{}", p.0 + p.1);
   ```

2. Can this compile?
   ```rust
   struct Test {
       name: String,
   }
   let t = Test { name: String::from("hi") };
   t.name = String::from("bye");
   ```

3. What’s the output?
   ```rust
   struct Marker;
   let _m = Marker;
   println!("Struct created");
   ```

---

## 🧠 Quiz Answers

1. ✅ This will print:
   ```
   15
   ```

2. ❌ This will not compile.
   - `t` is not mutable.
   - Fix it with `let mut t = Test { ... }`.

3. ✅ This will print:
   ```
   Struct created
   ```
