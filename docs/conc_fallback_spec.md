# 📦 ConC Fallback and Raw Input Handling Specification

## Overview

This document defines the fallback mechanisms and raw input handling strategies for the ConC (Conversation Compression) system. It outlines how the ConC parser and encoder should behave when encountering unknown or malformed input, offering multiple strategies for developers and systems to control behavior under failure conditions.

---

## 🧠 Motivation

Language is messy. Users misspell words, use obscure jargon, or input malformed strings. ConC, as a symbolic compression format, must provide clear mechanisms for:

- Gracefully handling unknown or invalid input
- Preserving user intention where possible
- Remaining deterministic and compressible
- Offering developers control over strictness and fallback behavior

---

## 🧩 Core Concepts

### 1. Fallback Modes

ConC supports three fallback modes for handling unknown or unresolvable input:

| Mode      | Behavior                                                                 |
|-----------|--------------------------------------------------------------------------|
| **Loose** | Silently skip invalid words. Input is ignored and not encoded.           |
| **Default** | Emit `𝗘𝗥𝗥𝗥` diagnostic symbol and preserve input where possible.         |
| **Strict** | Force deterministic encoding using the Raw Word Series (Summary 2) or raw literal tokens. |

---

### 2. Diagnostic Symbol: `𝗘𝗥𝗥𝗥`

A reserved symbolic word used to denote that ConC parsing **failed** on the current word.

- Appears only in **Default mode**
- May include optional metadata:
  ```json
  {
    "word": "definately",
    "base_word": "𝗘𝗥𝗥𝗥",
    "zone": "err",
    "metadata": {
      "guessed_intent": "definitely",
      "confidence": 0.78
    }
  }
  ```

---

### 3. Raw Input Keywords

Special keywords indicate that following tokens represent uninterpreted or literal content.

| Keyword | Meaning                                                                |
|---------|------------------------------------------------------------------------|
| `RRAW`  | Raw block mode. Parser disables all interpretation until end marker.   |
| `NRAW`  | Native words follow. Words are preserved as-is (e.g., plain UTF-8).    |
| `CRAW`  | ConC-shaped (4-char) tokens follow, but are **not decoded** semantically. |

This supports hybrid content, debugging, lossless logging, and interop with other symbol systems.

Example:
```
... 𝚫𝚼𝙨𝙈 RRAW CRAW xYz7 CRAW gGm2 RRAW_END ...
```

---

### 4. Raw Word Series (Summary 2 Integration)

In Strict mode, ConC may choose to encode unknown words as a **multi-token spelling** using a letter/syllable map.

- First token:
  - Presentation: `𝙇` (LETTER_SERIES_START)
  - Tone: number of tokens (e.g. `𝟞`)
- Middle tokens:
  - Presentation: `𝙇`
  - Tone: position (e.g. `𝟚`)
- Final token:
  - Presentation: `𝙇`
  - Tone: `𝙀` (LETTER_SERIES_END)

This enables structured spelling encoding of unknown inputs in a way that:
- Can be reversed
- Allows partial decoding
- Preserves sequence structure

---

## 🧰 Pre-Parse Spellchecking (Optional)

Before applying fallback logic, the ConC encoder may optionally use an external spellchecker or LLM model to correct malformed words:

```ts
correct(word: string) => Option<string>
```

If the spellchecker provides a valid correction that matches a known ConC entry, that entry is used. Otherwise, fallback modes proceed as normal.

---

## 🧠 Design Principles

- **Symbol-First:** All fallback is expressed in valid ConC-compatible tokens
- **Explicitness:** No silent fallbacks unless `Loose` mode is chosen
- **Forward Compatibility:** Raw or error-tagged tokens can be reprocessed later
- **Parser Resilience:** Raw/literal tags enable graceful handling of foreign or mixed formats

---

## 🛠 Next Steps

- Define the character set and structure for letter-series encoding
- Implement `--strict`, `--loose`, `--default` modes in CLI tools
- Add parser rules to recognize and process `RRAW`, `NRAW`, `CRAW`
- Integrate spellchecker as optional pre-pass hook

