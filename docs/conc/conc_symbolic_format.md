# ConC Symbolic Architecture: Meta Keywords, AABB Structure, and Model Memory

This document captures the official architectural direction for ConC’s symbolic format — focused on compression, expressiveness, and machine-executable meaning.

---

## ✅ AABB Encoding Format

All ConC words are encoded as a pair of base words:

```
AABB
```

- `AA` = Symbol representing a natural word or token
- `BB` = Second symbol representing another word or instruction
- Each half is a ConC base word (2-character symbol)
- Special symbol (e.g., `𝗇υⅼⅼ`) used when pairing with nothing (odd word count)

**Benefits**:
- Doubles compression ratio (2 words per 1 ConC token)
- Implies structure/grouping
- Increases predictive efficiency (fewer tokens needed)

---

## ✅ Meta-Keyword Representation

Instead of using structural digits to encode tone, presentation, etc., ConC supports meta instructions as special ConC words.

**Example**:
```text
::meta-tone-neutral:: ::meta-tone-angry::
You are not welcome here
↓
[neutral] [angry] [you are] [not welcome] [here]
↓
ΞΑΞΒ | ΑΑΒΒ | ΓΓΔΔ | ...
```

Meta-keywords include:

| Type         | Purpose                       |
|--------------|-------------------------------|
| `META_TONE_ANGRY`   | Indicates emotional tone     |
| `META_SCOPE_GLOBAL` | Scoping rule for instruction |
| `META_ERRR`         | Explicit error signaling     |
| `META_PRESENT_EMPH` | Marks emphasis presentation  |

These are encoded using standard base word symbols, but with reserved semantic meaning.

---

## ✅ Special Characters

ConC explicitly maps punctuation and special characters using reserved base words.

| Symbol | Meaning         |
|--------|------------------|
| `ΞΞ`   | `!` (exclamation) |
| `ΞΦ`   | `?` (question)    |
| `ΞΓ`   | `@` (at symbol)   |
| `ΞΔ`   | newline (`\n`)   |

This enables high-fidelity compression of:
- Punctuation
- Emojis
- Markdown
- Syntax

---

## ✅ Model-Generated Symbol Memory

ConC will support zones where models can:

- Define new symbolic entries
- Think using internal compressed symbols
- Reference learned abstractions

This allows ConC to function as a **machine code for cognition**, enabling LLMs to:
- Embed thoughts in compact ConC form
- Reuse them across reasoning steps
- Serialize working memory symbolically

### Escape Sequences

Models may use `::escape::` or other meta-words to trigger internal ConC-only blocks, e.g.:

```text
::escape:: run_reasoning_chain
::model:: IDEA_X
```

---

## Summary

| Feature                | Mechanism                         |
|------------------------|-----------------------------------|
| Symbol pairing         | `AABB` base word concatenation    |
| Metadata               | Meta ConC words (tone, scope, etc.) |
| Punctuation/symbols    | Reserved base words (`ΞΞ`, etc.)  |
| LLM memory             | Model-owned ConC symbol zone      |
| Execution escape       | `::escape::` style meta tokens    |

Next steps: define symbolic zones and ranges, finalize reserved entries.
