# Crucible

Non-functional testing for Polkadot SDK / Substrate networks, with a reusable core.

## Status

This project is an architecture and Rust API draft, not a working framework. The source sketches
contain unresolved types and signatures and do not yet compile. Moving them here does not resolve
or finalize those interfaces.

## Architecture

The guiding user experience is:

```text
Declaration → Execution → Evaluation
Evaluation: extract → validate → report
```

The test author supplies investigation-specific behavior; the core manages execution mechanics
and lifecycle. See [the architecture decisions](docs/architecture.md) for the agreed boundaries,
domain vocabulary, and rules for extending the design.

## Layout

| Path | Purpose |
| --- | --- |
| `src/lib.rs` | Module declarations and public re-exports |
| `src/declaration.rs` | Test declarations, run plans, and source-set contract |
| `src/execution.rs` | Load generation, recording/retrieval, and execution contracts |
| `src/evaluation.rs` | Evidence access, extraction, validation, and reporting contracts |
| `scratch/` | Uncompiled API sketches and experiments |
| `docs/architecture.md` | Agreed architecture decisions |

## Origin

The draft was extracted from `polkadot-pop-e2e/stress/crates/framework`. The existing stress runner,
its findings, and its investigation notes remain in `polkadot-pop-e2e`.
