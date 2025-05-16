
# 🧩 Step 4: Control Flow

Rust has powerful, expressive control flow features including `if`, `match`, and loop constructs (`loop`, `while`, `for`). Let’s explore them one by one.

---

## 🔀 1. `if` Expressions

```rust
let number = 6;

if number % 4 == 0 {
    println!("Divisible by 4");
} else if number % 3 == 0 {
    println!("Divisible by 3");
} else {
    println!("Not divisible by 3 or 4");
}
```

- Conditions must be boolean.
- `if` can be used as an expression to assign values:

```rust
let condition = true;
let number = if condition { 5 } else { 10 };
```

---

## 🎯 2. `match`

```rust
let number = 2;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}
```

- `_` is the catch-all case.
- Can match ranges and multiple patterns:

```rust
match number {
    1..=5 => println!("Between 1 and 5"),
    6 | 7 => println!("Six or seven"),
    _ => println!("Other"),
}
```

---

## 🔁 3. `loop`

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
println!("Result: {}", result); // 20
```

---

## 🔁 4. `while`

```rust
let mut n = 3;
while n != 0 {
    println!("{}!", n);
    n -= 1;
}
println!("LIFTOFF!");
```

---

## 🔁 5. `for`

```rust
let a = [10, 20, 30, 40];
for element in a {
    println!("Element is: {}", element);
}
```

```rust
for number in 1..4 {
    println!("{}", number); // 1, 2, 3
}

for number in (1..=3).rev() {
    println!("{}", number); // 3, 2, 1
}
```

---

## 🛠️ Important Concepts

| Tool       | Use Case                                |
|------------|------------------------------------------|
| `if`       | Conditional branching                    |
| `match`    | Pattern matching with values/ranges      |
| `loop`     | Infinite loop with optional value return |
| `while`    | Conditional loop                         |
| `for`      | Iterating over iterables and ranges      |
| `break`    | Exit a loop                              |
| `continue` | Skip current iteration                   |
| `.rev()`   | Reverse a range or iterator              |
| `1..5`     | Exclusive range: 1 to 4                  |
| `1..=5`    | Inclusive range: 1 to 5                  |

---

## 🧪 Quiz

1. What does the following print?
   ```rust
   let x = 3;
   let result = if x < 5 { x + 1 } else { x - 1 };
   println!("{}", result);
   ```

2. What will this code do?
   ```rust
   let mut i = 0;
   loop {
       i += 1;
       if i == 3 {
           break;
       }
       println!("{}", i);
   }
   ```

3. What is the output?
   ```rust
   for n in (1..4).rev() {
       print!("{} ", n);
   }
   ```

---

## 🧠 Quiz Answers

1. The output is:
   ```
   4
   ```
   - Since `x = 3`, and `3 < 5`, the `if` branch runs → `x + 1 = 4`.

2. The output is:
   ```
   1
   2
   ```
   - It loops until `i == 3`, but breaks before printing `3`.

3. The output is:
   ```
   3 2 1 
   ```
   - `.rev()` reverses the range. `1..4` means 1 to 3 (exclusive of 4).
   - So `.rev()` gives 3, 2, 1.
