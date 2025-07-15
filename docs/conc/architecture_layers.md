
# 🧠 ConC Runtime Architecture

This document outlines the layered architecture and design philosophy for the ConC (Conversation Compression) runtime, particularly focused on decoding pipelines. The architecture is designed using **First Principles** and **Layer Purity**, and aims to serve as a foundation for a symbolic language that functions like an "LLM Assembly Language."

---

## 🧬 High-Level Pipeline

```text
User Input (File / String / CLI)
       ↓
   [IOHandler] — Loads and emits data
       ↓
[DataTransport] — Moves data internally
       ↓
[DataSanitizer] — Prepares valid ConC words
       ↓
    [Decoder] — Pure word-to-symbol transformation
       ↓
     [Logger] — Reports diagnostics (side channel)
       ↓
   Output / Result / Error
```

---

## 🧾 Layer Definitions

### 1. 📦 IOHandler (Boundary I/O)

**Role:** Handles data entering and exiting the system.

**Responsibilities:**
- ✅ Open and read files
- ✅ Write decoded results
- ✅ Optionally read from `stdin`, write to `stdout`
- ❌ *No internal streaming*
- ❌ *No error logging or sanitization*

This layer is the **skin** — it moves data across the program boundary, but does not interpret or route it internally.

---

### 2. 🔁 DataTransport (Internal Streaming)

**Role:** Moves buffers, lines, or tokens between system components.

**Responsibilities:**
- Stream lines or word tokens into the system
- Abstracts iterators or async streams
- Decouples input format from processing logic

---

### 3. 🧼 DataSanitizer (Pre-decoder Cleaning)

**Role:** Normalizes raw input into clean ConC-ready form.

**Responsibilities:**
- Trim whitespace
- Split lines into 4-character ConC words
- Filter blank or malformed entries
- Validate word structure and symbol form

---

### 4. 🧠 Decoder (Core Transformation Logic)

**Role:** Pure function to convert a ConC word into a native word using a `SymbolMap`.

**Responsibilities:**
- Accept a 4-character ConC word (`&str`)
- Extract 2-symbol `base_word`
- Use `SymbolMap` to resolve base_word → index → native word
- Stub/ignore `presentation` and `tone` for now

**Traits:**
- Stateless
- Fully testable
- Format-agnostic

---

### 5. 📋 Error Model (Structured Handling)

**Role:** Scoped, typed errors for each layer.

**Strategy:**
- Each layer has its own error enum (`IOHandlerError`, `DataSanitizerError`, `DecodeError`)
- All implement `std::error::Error` via `thiserror`
- Can be unified into a global `ConCError` for CLI or runtime use

**Philosophy:** Local clarity + global composability.

---

### 6. 📝 Logger (Side-channel Reporting)

**Role:** Records diagnostics, not control flow.

**Responsibilities:**
- Log file-level issues
- Log skipped or malformed words
- Track decoding warnings or presentation mismatches
- Optional strict mode logging

---

## 🧭 Design Philosophy

### ✅ Layer Purity
> Each component should have **one purpose** and no upstream/downstream knowledge unless explicitly passed.

Prevents "gravity leaks" where UI or CLI behavior influences core logic.

### ✅ First Principles
> Start from fundamentals: a ConC word is a symbolic instruction. Decoding is instruction decoding. Clean, reversible, deterministic.

This mirrors compiler design and instruction set architecture.

### ✅ Symbolic Model
> ConC is being built as a symbolic compression format *and* a potential low-level communication language for LLMs.

The design choices must reflect:
- Predictable logic
- Human-inspectable results
- Efficient symbol encoding and decoding
- Clean extensibility

---

## ✅ Layer Ownership Summary

| Layer           | Owns                                       | Forwards to             |
|----------------|---------------------------------------------|--------------------------|
| IOHandler       | File access, CLI input/output              | DataTransport            |
| DataTransport   | Streaming text/buffers                     | DataSanitizer            |
| DataSanitizer   | Tokenizing and validating 4-char words     | Decoder                  |
| Decoder         | Converting ConC → native using SymbolMap   | (returns result)         |
| Error Model     | Structured error tracking per-layer        | Logger / Runtime         |
| Logger          | Warnings, diagnostics, context             | Human/user channel       |

---

## 🚧 Next Considerations

- Implement each layer with strict interfaces
- Introduce a config for strict/lenient decode modes
- Add support for tone/presentation variants (future)
- Allow streaming or real-time decoding (CLI pipes, WebAssembly)
