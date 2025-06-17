# ConC.GPT

**ConC.GPT** (Conversation Compression for GPT) is a symbolic compression system for natural language, designed for efficient storage and processing by LLMs. It maps every word in the English dictionary to a unique 3-character code using a 2-byte Unicode character set — enabling lossless and highly compact representation of large text corpora.

---

## 🌐 Project Overview

This project originated as a solution for fitting large source code (e.g. `binutils`, `emscripten`) into a constrained token space for LLM analysis. Instead of traditional compression, it introduces a **symbolic reference map**, replacing words with fixed-length symbols for consistency, compression, and machine-native decoding.

---

## 🧠 Key Concepts

- **Fixed-Length Symbols**: Each word is mapped to a 3-character code using extended Unicode (2-byte) characters.
- **Non-Human Readable**: Optimized for GPTs, not people.
- **Phrase-Level Compression**: Enables future expansions like multi-word encoding and control syntax.

---

## 📁 Repository Contents

- `conc_dict_00001_10000.jsonl` → `conc_dict_230001_240000.jsonl`  
  → 24 `.jsonl` files with 10,000 entries each (word → symbol)

- `conc_dict_index_with_symbols.jsonl`  
  → Index of first/last word + symbol in each file (for fast lookup)

- `conc_next_steps.md`  
  → Development roadmap

---

## ⚙️ Goals

- Compress natural language to maximize token efficiency
- Enable lossless round-trip translation: text → symbols → text
- Build an open, extensible framework for LLM-native encoding

---

## 🛠 Planned Tools

- **Rust**: For encoding, decoding, decompression tools
- **WASM**: Future browser-based LLM integration
- **Python**: Prototyping and batch generation (initial phase)

---

## 🚀 Status

✅ Dictionary mapped  
✅ Symbol range verified  
✅ Encoding exported  
🧩 Phrase compression, decoding, and integration up next

---

## 📄 License

To be determined. (MIT or Apache 2.0 recommended)

---

Built with 🤖 by a human who didn't want their EC2 instance to melt.
