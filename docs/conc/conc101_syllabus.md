
# 🧠 ConC 101 Syllabus: From Compression to Machine Code for LLMs

## 📘 Course Overview

This course explores the transformation of ConC from a symbolic compression format into an executable machine code for language models. By treating symbolic words not merely as compressed placeholders, but as cognitively significant instructions, we can build models that reason symbolically, act deliberately, and reflect internal semantic state.

---

## 📅 Lecture Plan

### Lecture 1: Compression vs Machine Code
- Compare traditional compression with LLM machine code
- Understand the difference between space-saving and behavior-invoking tokens
- Case study: Linux Kernel Expert Model
- **Assignment 1**: Design symbolic representations for 5 key domain concepts

---

### Lecture 2: Designing Executable Symbols
- How to assign meaning to symbols beyond compression
- Symbol zones and role-based encoding
- Tone/presentation as flags, not decoration
- Preparing semantic metadata (tags, roles, behavior classes)

---

### Lecture 3: Training with Symbol Awareness
- Building a curriculum for symbol-based learning
- Phased training strategy: base_word → tone → presentation
- Teaching models to prefer ConC over raw input
- Dataset structuring for symbolic fidelity

---

### Lecture 4: Executing Symbolic Reasoning at Runtime
- How ConC tokens guide inference pathways
- Prompt structures that act as cognitive programs
- Recall and memoization through symbol reuse
- Context shaping with instruction-layer patterns

---

### Lecture 5: Runtime Symbolic APIs (Compression + Execution)
- Designing APIs and toolkits to encode/decode ConC in apps
- Using ConC in constrained environments (WASM, CLI)
- Exposing ConC behaviors via runtime hooks
- Context streaming: symbolic stacks and scopes

---

### Lecture 6: Building an LLM OS Kernel
- What symbolic execution enables at scale
- ConC as a foundation for LLM operating systems
- Managing symbolic memory, modules, and processes
- Future directions: code synthesis, symbolic cognition, co-training

---

## ✅ Course Requirements

### Required Before Proceeding:
Complete the assignment from **Lecture 1**:

> **Pick a domain**, choose 5 core phrases, and for each:
> - Define a compact ConC symbol
> - Annotate its **semantic or behavioral role**

You must treat these symbols as not just abbreviations, but as units of thought the model should act on.

---

## 🧠 Goals By End of Course

You will be able to:
- Build executable symbol sets for domain-specific models
- Train LLMs to respond to symbolic instruction tokens
- Design prompts and systems that reason with symbolic layers
- Construct a symbolic runtime: compact, expressive, and deterministic
