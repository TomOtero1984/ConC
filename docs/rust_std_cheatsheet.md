# 🦀 Rust Standard Library Cheat Sheet – Essentials

## 📢 Output & Debugging

| Macro        | Description                              |
|--------------|------------------------------------------|
| `println!()` | Print to `stdout`                        |
| `eprintln!()`| Print to `stderr`                        |
| `dbg!(...)`  | Print value and file/line (for debugging)|

---

## 🔁 Loops & Control

| Macro        | Description                              |
|--------------|------------------------------------------|
| `for`        | Iterate over anything iterable           |
| `loop`       | Infinite loop                            |
| `if let`     | Match one pattern (e.g. `Ok(_)`)         |
| `while let`  | Keep looping while pattern matches       |
| `match`      | Full enum matching (e.g. `Result`, `Option`) |

---

## 📦 Option / Result Helpers

| Function / Macro     | Description                                  |
|----------------------|----------------------------------------------|
| `.unwrap()`          | Get inner value or panic                     |
| `.expect("msg")`     | Same as unwrap, but with custom message      |
| `?`                  | Propagate `Err`/`None` upwards               |
| `Result::ok()`       | Convert to `Option`                          |
| `Option::unwrap_or()`| Default if `None`                            |

---

## 🧵 Strings and Vectors

| Function / Macro     | Description                                  |
|----------------------|----------------------------------------------|
| `format!()`          | Like `printf`, returns a `String`            |
| `.push_str()`        | Append to a `String`                         |
| `.to_string()`       | Convert anything `Display` to a `String`     |
| `.into_bytes()`      | Convert a `String` into `Vec<u8>`            |
| `.as_bytes()`        | Get a string slice as byte slice             |
| `.split_whitespace()`| Split a string on spaces/tabs/newlines       |

---

## 📚 File and IO

| Function (from `std::fs`, `std::io`) | Description                    |
|--------------------------------------|--------------------------------|
| `fs::read_to_string(path)`          | Reads a whole file into a `String` |
| `fs::write(path, contents)`         | Writes a string to a file      |
| `BufReader::new(file)`              | Buffered reader for efficient `.lines()` |
| `stdin().read_line(&mut String)`    | Read from user input           |
| `stdout().write_all(&[u8])`         | Manual write to stdout         |

---

## 🧠 Utility Macros

| Macro        | Description                              |
|--------------|------------------------------------------|
| `vec![]`     | Create a vector                          |
| `assert!()`  | Panic if condition is false              |
| `assert_eq!()`| Panic if two values aren't equal        |
| `todo!()`    | Marks code as not implemented yet        |
| `unimplemented!()` | Same, but for APIs that compile     |

---

## 🕵️ Iterators & Collections

| Function / Trait        | Description                              |
|-------------------------|------------------------------------------|
| `.map()`, `.filter()`   | Transform or filter iterators            |
| `.collect::<Vec<_>>()`  | Turn iterator into vector                |
| `.enumerate()`          | Add index to iteration                   |
| `.flatten()`            | Flatten nested iterators                 |
| `.zip()`                | Pair up two iterators                    |
