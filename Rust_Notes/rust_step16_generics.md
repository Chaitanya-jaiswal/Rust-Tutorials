
# 🧩 Step 16: Generics

Generics let you write flexible, reusable code that works with different types.

---

## 🧰 1. Basic Generic Function

```rust
fn print_value<T>(val: T) {
    println!("Value received!");
}
```

- `T` is a placeholder for any type

---

## 📦 2. Generics with Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

```rust
let int_point = Point { x: 1, y: 2 };
let float_point = Point { x: 1.1, y: 2.2 };
```

---

## ✨ 3. Generics with Multiple Types

```rust
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

let mix = MixedPoint { x: 3, y: 4.5 };
```

---

## 🧠 4. Generic Methods

```rust
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
}
```

```rust
impl Point<f64> {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}
```

---

## 📍 5. Trait Bounds on Generics

```rust
fn print_debug<T: std::fmt::Debug>(val: T) {
    println!("{:?}", val);
}
```

### Cleaner with `where`

```rust
fn print_debug<T>(val: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", val);
}
```

---

## 🛠️ Common Patterns

| Pattern                     | Purpose                                    |
|-----------------------------|--------------------------------------------|
| `fn <T>(x: T)`              | Generic function                          |
| `struct Name<T>`            | Generic struct                            |
| `impl<T> Struct<T>`         | Generic implementation                    |
| `T: Trait`                  | Trait bound (must implement Trait)        |
| `where T: Trait`            | Cleaner multi-bound declaration           |

---

## 🧪 Quiz

1. Will this compile?

```rust
fn echo<T>(x: T) -> T {
    x
}
```

2. Can this work?

```rust
fn compare<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

3. What's the output?

```rust
#[derive(Debug)]
struct Boxed<T> {
    value: T,
}

fn main() {
    let b = Boxed { value: 42 };
    println!("{:?}", b);
}
```

---

## 🧠 Quiz Answers

1. ✅ Yes, this is valid and returns the same value back.

2. ✅ Yes, but it requires that `T` implements `PartialOrd` so `>` works.

3. ✅ Output:
```
Boxed { value: 42 }
```
Because of `#[derive(Debug)]`, the struct can be printed with `{:?}`.
