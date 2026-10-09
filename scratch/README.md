# Scratch playground

This directory holds uncompiled API sketches and compiler experiments. Cargo does not include these
files in the library or discover them as binary targets.

The current candidate interfaces live in the three phase modules under `src/`. Keep alternative
designs here until we choose to integrate them; scratch code does not establish architecture
agreement.

`typed_sources.rs` preserves the initial run-plan builder sketch. Its identifiers and methods are
not implemented yet, and the snippet is not a standalone Rust program.
