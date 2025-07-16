# 🧠 ConC.GPT — A Symbolic Machine Language for LLMs

**ConC.GPT** (Conversation Compression for GPT) is a symbolic language for compactly encoding and executing meaning inside language models. 
Originally a compression system, ConC has evolved into a **cognitive assembly language**; a way to express intent, tone, 
and semantics in 4-character symbolic words.

---

## 🚀 Vision: From Compression to Machine Code

ConC is a **compact symbolic instruction** that can guide LLM reasoning, modify tone or behavior, and encode semantic roles.

We’re building:
- ✅ A **universal symbolic format** for GPT-native thinking
- ✅ A **runtime pipeline** to decode and execute symbolic input
- ✅ A foundation for **LLM operating systems**, memory, and symbolic logic

---

## 🔤 ConC Word Structure

Each ConC word is exactly 4-Unicode characters:

    [base_word][presentation][tone]

#### Part Length Meaning
- _base_word_: 2-chars core meaning (e.g. "apple" → Ă훯)
- _presentation_: 1-char visual form (e.g. lowercase → Ā)
- _tone_: 1 char contextual/affective tone (e.g. neutral → Ā)

All characters are drawn from a curated GPT-safe Unicode set, optimized for token efficiency and directional safety.

### 🍎 Example:

The word "apple" with lowercase presentation and neutral tone is encoded as:

    Ă훯ĀĀ

This corresponds to the JSON entry:

``` json
{
"word": "apple",
"base_word": "Ă훯",
"presentation": "lowercase",
"base_presentation": "Ā",
"tone": "neutral",
"base_tone": "Ā"
}
```
Each ConC word is compact, symbolic, and designed for fast runtime decoding or execution.

---

## 🧱 Runtime Architecture

ConC’s runtime is designed using first principles and strict layer separation:

``` mermaid
flowchart TD
    A[User Input<br/>File / String / CLI]
    B[IOHandler<br/>Loads and emits data]
    C[DataTransport<br/>Internal stream/iterator control]
    D[DataSanitizer<br/>Filters & splits into valid ConC words]
    E[Codex<br/>Pure word-to-meaning transformation]
    F[Logger<br/>Side-channel diagnostics]
    G[Output / Result / Error]

    A --> B --> C --> D --> E --> G
    E --> F
```

Each layer is testable, replaceable, and symbolically pure.

---

## 🧰 Tooling & Implementation

Tool / Feature	Status
Symbol dictionary	✅ Complete (JSONL format)
Base word encoding	✅ Stable, zone-aware
CLI encoder (Rust)	✅ Working
Decoder runtime	🛠 In progress
Presentation/tone logic	✅ Implemented in format
Zone partitioning	🛠 Design in progress
WASM integration	🛠 Planned

All code is being written in Rust for speed and reliability. A WASM build is planned for browser-based execution.

---

## 🧭 Roadmap
- Refactor to layered architecture
  - [ ] Logger
  - [ ] IOHandler
  - [ ] DataTransport
  - [ ] DataSanitizer
  - [ ] Codex
- Add zone-aware allocator
- CLI decoder and runtime stack
- Full round-trip compression → execution
- Train ConC-aware expert model

---

## 📄 License

TBD — likely MIT or Apache 2.0

---

## 🙋 About

This project was built by a human who didn’t want their LLM tokens to melt.

Built with defiance, Unicode, and an unhealthy obsession with symbol sets.
