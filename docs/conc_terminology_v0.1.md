
# ConC Terminology (v0.1)

This document defines the core terminology used in the **ConC** system — a compressed language designed for machine-native communication, symbolic parsing, and token-efficient encoding.

---

## 📘 Qualifiers

These qualifiers distinguish between terms in their *natural* vs. *compressed* forms.

### Original / Natural Language
- `natural`
- `original`
- `standard`
- `decoded`
- `restored`
- `native`
- `source`

### ConC Language
- `generated`
- `encoded`
- `compressed`
- `parsed`
- `processed`

> **Note:** The name “ConC” refers to the compressed language used in the ConC.GPT system.  
> A natural language name (e.g. “English”) refers to the source language.  
> “ConC-English” refers to the ConC form generated from English.  
> Any term qualified with *ConC* refers to the processed/encoded equivalent.

---

## 🧩 Core Terms

### `Character`
Neutral reference to either a **letter** or **symbol** from the original or processed language.

### `Charset`
An indexed selection of Unicode characters used to represent the ConC **alphabet**.

### `Symbol`
An indexed Unicode character used in ConC to represent a word or phrase.
- Example: `index[0] == "Ā"`
- If the source language uses symbols, qualifiers must distinguish between *natural* and *ConC* symbols.

### `Letter`
A character from the alphabet of a natural language, unless otherwise qualified.

### `Word`
A neutral reference to any combination of characters — applies to both natural and ConC language unless qualified.

### `Index`
The position of a **word** or **character** within a collection, dataset, or map.

### `Map`
A collection or reference that associates natural language with ConC, used for encoding or decoding.

---

## 🧱 ConC Word Structure

### 🔹 Base Word
- A **2-character** ConC sequence used to represent a mapped word.
- Formed from **two indexed ConC symbols**.
- Always appears as the **first two characters** of a ConC word.

### 🔹 ConC Word
A **4-character** token used to represent a compressed expression.

**Structure:**
```
[ base1 ][ base2 ][ meta1 ][ meta2 ]
```
- `base1` and `base2`: The 2-symbol base word
- `meta1`: Presentation metadata (e.g. formatting, case, plurality, tense)
- `meta2`: Tone, intent, or discourse context (e.g. emphasis, sarcasm, emotion)

This 4-character format is strict and non-negotiable — it is one of the foundational features of ConC.

---

## 🔖 Example Distinctions

| Term              | Natural Language Example | ConC Equivalent           |
|-------------------|--------------------------|----------------------------|
| Character         | `a`, `b`, `@`, `é`       | `ⵣ`, `𝞅`, `🝖`              |
| Charset           | *N/A*                    | `[ "Ā", "Ă", "Ą", ... ]`   |
| Symbol            | *N/A*                    | `"ⵣ"`                      |
| Letter            | `a`, `b`, `c`            | *Not used in ConC*         |
| Word              | `"run"`                  | `"ⵣ𝞅𝞃𝝷"`                   |
| Index             | `3` (for word)           | `3` (for symbol)           |
| Map               | `"run" → ⵣ𝞅"`            | Reverse map for decoding   |

---

End of document.
