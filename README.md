
# Crucible

A non-functional testing framework for Polkadot SDK networks.

Define a workload, record how the system behaves, and evaluate the results
through test-specific and reusable node checks.

**Status:** Early architecture and Rust API draft. Not yet a working framework.

## Goals

Make non-functional tests easy to create, with reusable checks, explicit
environment requirements, and consistent, readable reports.

![Crucible goals](static/goals.svg)

## How it works

1. **Declaration** — Specify environment requirements, recording sources,
   workload, and checks.
2. **Execution** — Verify readiness, prepare the test, apply load, and record
   observations.
3. **Evaluation** — Extract relevant data, validate it against the test’s
   criteria, and produce one report.

Test authors define the investigation. Crucible handles the shared execution
mechanics and provides reusable node checks.

![Crucible architecture overview](static/architecture-overview.svg)

## Design

See the [architecture decisions](docs/architecture.md) for the agreed direction.
The Rust interfaces in `src/` and experiments in `scratch/` are still evolving.
