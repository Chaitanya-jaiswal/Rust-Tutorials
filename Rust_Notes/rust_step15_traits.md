
# 🧩 Step 15: Traits — In Depth

Traits define shared behavior across types. Think of them like interfaces in other languages.

---

## 📌 1. What is a Trait?

```rust
trait Greet {
    fn say_hello(&self);
}
```

Defines a contract: any type implementing `Greet` must define `say_hello()`.

---

## 📌 2. Implementing a Trait

```rust
struct Human;
struct Robot;

impl Greet for Human {
    fn say_hello(&self) {
        println!("Hi, I'm a human!");
    }
}

impl Greet for Robot {
    fn say_hello(&self) {
        println!("01001101 Hello");
    }
}
```

---

## 📌 3. Using Traits in Functions

### Method 1: `impl Trait`

```rust
fn greet_entity(entity: &impl Greet) {
    entity.say_hello();
}
```

### Method 2: Trait Bound

```rust
fn greet<T: Greet>(entity: &T) {
    entity.say_hello();
}
```

### Method 3: Where Clause

```rust
fn greet<T>(entity: &T)
where
    T: Greet,
{
    entity.say_hello();
}
```

---

## 📌 4. Default Method Implementations

```rust
trait Hello {
    fn hello(&self) {
        println!("Hello from default!");
    }
}

struct Cat;
impl Hello for Cat {}

struct Dog;
impl Hello for Dog {
    fn hello(&self) {
        println!("Bark bark!");
    }
}
```

---

## 📌 5. Traits with Multiple Methods

```rust
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "Circle"
    }
}
```

---

## 📌 6. Trait Objects (`dyn Trait`)

```rust
fn greet_dyn(greeter: &dyn Greet) {
    greeter.say_hello();
}
```

Use when the type isn’t known until runtime.

---

## 📌 7. Common Built-in Traits

| Trait       | Use case                                |
|-------------|------------------------------------------|
| `Debug`     | Enables `println!("{:?}", ...)`          |
| `Clone`     | For `.clone()`                          |
| `PartialEq` | Enables `==` comparison                 |
| `Default`   | Used to create default values            |

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

---

## 🧪 Quiz

1. Will this compile?

```rust
trait Speak {
    fn say(&self);
}

struct Bird;

impl Speak for Bird {}

fn main() {
    let b = Bird;
    b.say();
}
```

2. How do you define a function that accepts anything that implements `Speak`?

3. Why use `dyn Trait`?

---

## 🧠 Quiz Answers

1. ❌ No. The `say()` method is not implemented in `impl Speak for Bird`.

2. ✅
```rust
fn speak_any<T: Speak>(val: &T) {
    val.say();
}
```

3. ✅ Use `dyn Trait` when the type is not known at compile time and you want **dynamic dispatch**.
