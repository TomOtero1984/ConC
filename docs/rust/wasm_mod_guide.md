# wasm_mod_guide.md

## ✅ Final Setup: `wasm-bindgen` + `wasm-pack` + clean module structure

### 📁 File Structure
```
conc_sh/
├── Cargo.toml
└── src/
    ├── lib.rs        # main library entry point
    └── wasm.rs       # WebAssembly interface
```

---

### 📦 Cargo.toml
```toml
[package]
name = "conc_sh"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]  # for wasm + native use

[dependencies]
wasm-bindgen = "0.2"
```

- ✅ `cdylib` is needed for WebAssembly (via `wasm-pack`)
- ✅ `rlib` is good for reusing this logic elsewhere (CLI, server, etc.)
- ❌ Removed `[lib] path = ...` — because it conflicted with standard layout

---

### 🧠 Code Design

#### `src/lib.rs`
```rust
pub fn translate(word: &str) -> String {
    format!("Translated: {}", word)
}

pub mod wasm;
```

- ✅ This is the main logic file (and crate root)
- ✅ `wasm` module included via `pub mod wasm;`

#### `src/wasm.rs`
```rust
use wasm_bindgen::prelude::*;
use crate::translate;

#[wasm_bindgen]
pub fn translate_word(word: &str) -> String {
    translate(word)
}
```

- ✅ `wasm_bindgen` exposed function for JS → WASM
- ✅ Uses `crate::translate` correctly

---

### 🧪 Usage with `wasm-pack`
```bash
wasm-pack build --target web --out-dir public
```

- ✅ Outputs `public/conc_sh.js` and `.wasm` file
- ✅ Works with this in your JS:

```js
import init, { translate_word } from './conc_sh.js';

async function run() {
    await init();
    console.log(translate_word("apple")); // Translated: apple
}
run();
```

---

### 💡 Key Insights
- You must use `crate::` or module paths (`mod wasm`) to reference other logic in the crate.
- You can’t treat `lib.rs` and `wasm.rs` as *competing entry points*. `lib.rs` is the root. Modules like `wasm.rs` must be included using `mod`.

