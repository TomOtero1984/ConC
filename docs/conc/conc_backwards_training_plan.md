# ConC Training Plan: Backwards Training Loop

**Saved on:** 2025-07-18 19:49:46

## Overview

This file outlines the Backwards Training Loop strategy for ConC.

## Concept

Instead of training a model to *encode* English into ConC, we train it to *decode* a known ConC word (base_word + tone + presentation) back into its English equivalent. Then we pass that output back through our deterministic Rust encoder and verify the output matches the original ConC word.

## Steps

1. **Input**: A known, valid ConC word (with base_word, tone, presentation fields).
2. **Model**: Predicts the original English word and associated metadata.
3. **Verification**:
    - Feed the predicted word + metadata into the Rust encoder.
    - Check if the output matches the input ConC word.
4. **Loss**: Based on match failure between original ConC and re-encoded result.

## Advantages

- Ensures symbolic *self-consistency*
- Decouples model from encoder logic
- Enables field-specific supervision (e.g. predict tone only)
- Works like a symbolic autoencoder with only the decoder being a model
- Allows deterministic, Rust-based verification and compression

## Requirements

- ConC encoder implemented in Rust
- Ground-truth ConC ↔ English dataset
- Training framework with round-trip verification capability

## Future Enhancements

- Add fuzzy decoding metrics (edit distance, synonym distance)
- Support partial recovery (tone only, presentation only)
- Expand to multi-word and phrase-based ConC structures
