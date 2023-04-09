# Rust
### Basic `for` loop w/ function
- Functions in Rust are declared with the `fn` keyword
- Function declarations are very like TypeScript (C-like), where the parenthesis follows the function name, followed by curly brackets
- In Rust, we declare immutable variables with `let`, and where mutability is required, we use `let mut`
- Declaring types for mutatable variables such as `even_arr: Vec<i32>` uses familiar angle brackets, and the `Vec::new()` static method creates a vector instance without creating an instance of the `Vec` type
- Using `|num: i32|` creates a closure (Rust lambda) which defines type `i32`
- The `for` loop in Rust uses the `in` keyword, and if an array iterable is passed, `.iter()` must be called
- In the `if` statement, we see again, familiar syntax, but the `is_even(*item)` function call uses dereferencing, on the reference passed - to ensure we get the actual value rather than a reference to the `i32` value
- This is the same again for the `.push(*item)`
- In the logging `{}` is used as a simple placeholder for logging the `item` value in this string position
- `{:?}` is used to print the contents of the `even_arr` itself, something as TS/JS devs we are very unfamiliar with as this provides a way to directly log (and debug) iterables

### A TypeScript developer learns Rust, one branch at a time

![rust-lang-ar21](https://user-images.githubusercontent.com/42226854/230785763-c339a1ef-446d-4864-8d73-981034624d4a.png)

