# 🧱 ConC Packet Structure (48-Bit)

## Overview
Each ConC symbol is represented by **4 Unicode characters** (each 2-3 bytes), giving a total of 48 bits.

```
+----------------+-------------------------+
|   16 bits      |         32 bits         |
| Flags / Meta   |      Symbol Index       |
+----------------+-------------------------+
       ↑                   ↓
     counts ↑           counts ↓
     (new features)     (dictionary growth)
```

---

## 🔐 Reserved Ranges

### 🔹 Symbol Index (32-bit)
| Range               | Purpose                            |
|---------------------|------------------------------------|
| `0xFFFFFFFF`        | `𝗘𝗥𝗥𝗥` — Fatal error               |
| `0xFFFFFFFE`        | `𝗙𝗔𝗜𝗟` — Recoverable failure       |
| `0xFFFFFFFD`        | `𝗪𝗔𝗥𝗡` — Parser warning           |
| `0xFFFFFFFC`–`0xFFFFFFF0` | System-only: checksums, debug tags |
| `0x00000010`–`0x00000000` | Reserved for future system use  |
| *(Everything else)* | Dictionary + Phrase Map           |

> 🧠  You still get ~4.2 billion usable values even after reserving a *ton*.

---

### 🔸 Flags / Meta (16-bit)
| Range           | Purpose                             |
|-----------------|-------------------------------------|
| `0x0000`        | `𝗇𝗌𝖾𝗍` — Not yet set (default)      |
| `0x0001`        | `𝗇υⅼⅼ` — Intentionally empty        |
| `0x0002`–`0x00FF` | Developer space (e.g. temp flags)  |
| `0x0100`–`0x7FFF` | User-defined control/meta behaviors |
| `0x8000`–`0xFFFF` | Reserved for future protocol use   |

---

## 🧠 Developer Considerations

- Reserved regions for checksums, debug, and internal parser flags.
- Clean room for developers to experiment.
- Separation of system vs user/meta behavior flags.
- Predictable growth: flags count upward, dictionary counts downward.

---

## ✅ Final Notes

- 48-bit packet = **compact**, **scalable**, and **token-friendly**
- Future-ready for multilingual dictionaries
- Reliable structure for deterministic compression

