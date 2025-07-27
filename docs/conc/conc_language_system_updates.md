
# ConC Language System Updates

## Symbol Format Update

ConC words now follow a 4-character symbolic format of the form **AABB**, where:
- `AA` = Index of base word 1 (first 2-char ConC symbol)
- `BB` = Index of base word 2 (second 2-char ConC symbol)

### Special Handling
- If there is an odd number of words, a special null word (e.g., `𝗇υⅼⅼ`) is used as a placeholder in the second slot.
- This placeholder can also imply sentence or segment completion.

---

## Tradeoff Summary: Compression vs Granularity

| Tradeoff                | Outcome                                          |
|------------------------|--------------------------------------------------|
| Granularity per token  | ❌ Reduced                                       |
| Compression            | ✅ Increased (2 words per token)                |
| Predictive efficiency  | ✅ Improved (fewer tokens per sequence)        |
| Semantic grouping      | ✅ Implicit and learnable by model             |

---

## ConC-glish Mode

A new hybrid escape mechanism:
- Wrap unrecognized or out-of-lexicon words with a special boundary, e.g.:
  ```text
  ⧼Hello⧽  => Interpreted as raw natural language
  ```
- These segments are:
  - Exempt from ConC decoding
  - Interpreted as natural language by the LLM
  - Useful for graceful degradation or mixed-format data

---

## Keyword Encoding & Signals

- Special keywords (ConC-encoded) will be reserved for:
  - Metadata
  - Tone markers
  - Presentation shifts
  - Control instructions
- These will exist in a reserved index range (not yet finalized)

---

## Reserved Symbolic Zones

- **User-defined words**
- **Model-inferred concepts**
- **Control/meta words**
- **Special characters and punctuation**

No zones have been finalized — but zoning is a fundamental part of future expansion and LLM interaction.

---

## Naming Notes

- The natural language scripting layer may adopt the name **Oyster** (from the previously shelved project).
- A future Perl superset language could be called **Clam**, continuing the shell theme.

