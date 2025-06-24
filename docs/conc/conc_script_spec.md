
# ConC Script v0.1 Specification

A minimal, LLM-safe, Unicode-compatible **meta-language** for instructing the ConC.GPT parser and tooling.

---

## 📦 Syntax Overview

All ConC Script blocks follow this **reserved structure**:

```
::_<
.directive
INSTRUCTION arg=value
... (optional lines)
 />_::
```

- **Container**: `::_<` starts the block, `/>_::` ends it
- **Directive**: Namespaced prefix like `.parser`, `.symbol`, `.debug`
- **Instructions**: Keyword/value pairs or simple verbs

These are **never encoded as symbols** and are parsed as meta-commands.

---

## 🎯 Syntax Rules

- Must start with `::_<` and end with `/>_::`
- All content inside must be valid UTF-8 text
- Case-insensitive by default (parser may override)
- Each instruction is placed on a new line
- First line must be a **dot-prefixed directive**

---

## 🧠 Built-in Directives and Examples

### `.parser`

Used to control parser behavior or metadata.

```txt
::_<
.parser
SET lang=en
SET mode=caseless
/>_::
```

### `.symbol`

Override or force a symbol-to-word mapping.

```txt
::_<
.symbol
OVERRIDE word="GPT" symbol="⍺⍺⍳"
/>_::
```

### `.debug`

Trigger debug or dev-time tools.

```txt
::_<
.debug
DUMP symbol_map
/>_::
```

---

## 🛡 Reserved Use

Use ConC Script blocks ONLY for dev/LLM tooling.  
They must **never be interpreted as part of normal input**.

---

## 🧩 Future Extensions

Future versions may support:
- `IF` and conditional logic
- `WITH` blocks for scoping
- `IMPORT` for shared metadata

---

## 🧬 Purpose

ConC Script provides a **safe, declarative interface** to communicate directly with GPT-based models, compression parsers, and developer tools.

It ensures:
- Safe passage of metadata
- Predictable instruction parsing
- Friendly developer control over encoded data streams

---

> Version: `v0.1`  
> Status: 🚧 Experimental
