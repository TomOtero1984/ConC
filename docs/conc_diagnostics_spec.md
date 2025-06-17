# ConC.GPT Diagnostic & Symbolic Logging Spec

This document defines the reserved symbolic values used by the ConC.GPT system to represent metadata states, diagnostics, and logging levels. All symbols are constructed using **2-byte Unicode full-width characters**, ensuring GPT token safety and visual clarity.

---

## 🔹 Null and Parsing States

| Symbol       | Keyword   | Unicode Representation     | Meaning                               |
|--------------|-----------|-----------------------------|----------------------------------------|
| `𝗇υⅼⅼ`       | `_null_`   | U+1D62F + Latin + U+1D4CF   | Intentional absence (e.g., “none”)     |
| `𝗇𝗌𝖾𝗍`       | `_nset_`   | U+1D62F + U+1D630 + U+1D5BE + U+1D631 | Not yet set / unprocessed               |

---

## 🔹 Diagnostic States

| Symbol       | Keyword   | Unicode Characters                      | Meaning                                 |
|--------------|-----------|------------------------------------------|------------------------------------------|
| `𝗘𝗥𝗥𝗥`       | `_err_`    | U+1D5EC U+1D5F1 U+1D5F1 U+1D5F1           | Critical processing error                |
| `𝗙𝗔𝗜𝗟`       | `_fail_`   | U+1D5ED U+1D5A0 U+1D5A2 U+1D5A5           | Non-critical failure                     |
| `𝗪𝗔𝗥𝗡`       | `_warn_`   | U+1D5FC U+1D5A0 U+1D5F1 U+1D5B3           | Warning or anomaly                       |
| `𝗜𝗡𝗙𝗢`       | `_info_`   | U+1D5EE U+1D5B3 U+1D5C9 U+1D5C8           | Informational message                    |
| `𝗗𝗕𝗨𝗚`       | `_dbug_`   | U+1D5D1 U+1D5AB U+1D5C6 U+1D5A6           | Debug-level message                      |
| `𝗩𝗕𝗢𝗦𝗘`       | `_vbose_`  | U+1D5F4 U+1D5AB U+1D5C8 U+1D5D0 U+1D5EC   | Verbose-level message                    |
| `𝗣𝗔𝗦𝗦`       | `_pass_`   | U+1D5F5 U+1D5A0 U+1D5D0 U+1D5D0           | Successful processing or validation      |

---

## 🧠 Usage Guidance

These symbols are reserved for internal state tracking and parser behavior in ConC.GPT. They are:

- Visually distinct and capitalized for easy spotting
- Semantically clear and explicitly defined
- GPT-safe and token-efficient
- Ideal for internal logging, debugging, metadata layers, or structured ConC scripting

---

## 🧪 Example: Embedded Metadata

```json
{ "word": "_warn_", "symbol": "𝗪𝗔𝗥𝗡" }
{ "word": "_pass_", "symbol": "𝗣𝗔𝗦𝗦" }
{ "word": "_nset_", "symbol": "𝗇𝗌𝖾𝗍" }
```

---

## 📌 Note

The characters used here are primarily from:
- **Mathematical Sans-Serif** ranges
- UTF-8 compatible, visually clean, and unlikely to be filtered or tokenized unpredictably

For custom directives, symbolic status, or new logging levels, use a consistent casing and encoding convention.