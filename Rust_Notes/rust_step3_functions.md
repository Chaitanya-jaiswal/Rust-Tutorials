
# 🧩 Step 3: Functions

Functions are a **core building block** in Rust. They allow you to **encapsulate logic**, **pass values**, and **return results** cleanly and safely.

---

## 📦 Declaring a Function

```rust
fn main() {
    println!("Hello from main!");
    greet();
}

fn greet() {
    println!("Hello from greet!");
}
```

- Functions are declared with the `fn` keyword.
- The entry point of every Rust program is the `main()` function.
- Rust uses **snake_case** for function names by convention.

---

## 📥 Parameters and Return Values

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

- Parameters must be typed.
- Return type follows `->`.
- Rust does **not** use `return` if the return expression is the **last line**:
  
  ```rust
  fn square(n: i32) -> i32 {
      n * n // No semicolon means it's the return value!
  }
  ```

- You can use `return` explicitly if needed:
  
  ```rust
  return x + y;
  ```

---

## 🚧 Statements vs. Expressions

- Rust distinguishes between **statements** and **expressions**.
- An expression **returns a value**.
- A statement **performs an action**, but returns nothing.

Example:
```rust
let x = {
    let y = 3;
    y + 1 // This is an expression → x will be 4
};
```

---

## ✅ Best Practices

- Use descriptive, lowercase function names with underscores.
- Always specify return types for clarity.
- Keep functions short and focused on a single task.

---

## 🛠️ Important Concepts / Methods

| Concept         | Explanation                                                   |
|-----------------|---------------------------------------------------------------|
| `fn`            | Keyword to define functions                                   |
| Return type     | Declared after `->`, must match returned expression           |
| Implicit return | No `return` keyword if last expression is returned            |
| Unit type       | `()` is the default return type if nothing is returned        |
| Statement       | Ends with `;`, doesn’t return value                           |
| Expression      | Does **not** end with `;`, returns a value                    |

---

## 🧪 Quiz

1. What does this function return?
   ```rust
   fn mystery() -> i32 {
       3 + 4;
   }
   ```

2. What will this print?
   ```rust
   fn test() -> i32 {
       let x = 2;
       let y = 3;
       x + y
   }
   fn main() {
       println!("{}", test());
   }
   ```

3. What’s the difference between:
   ```rust
   let a = { let b = 2; b + 1 };
   let a = { let b = 2; b + 1; };
   ```

---

## 🧠 Quiz Answers

1. The function returns `()` (unit type) because `3 + 4;` is a **statement**, not an expression (it ends with a semicolon and doesn't return the value).

   ✅ Correct answer: `()` (not `7`)

2. The function returns `5`, so the program will print:
   ```
   5
   ```

3. The difference lies in **expression vs. statement**:

   ```rust
   let a = { let b = 2; b + 1 };  // Returns 3 (expression, no semicolon)
   let a = { let b = 2; b + 1; }; // Returns () (statement, due to semicolon)
   ```

   ✅ The first returns a value (`3`), the second returns unit `()`.
