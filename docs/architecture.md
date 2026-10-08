# Stress-test architecture decisions

## Purpose and guardrails

This is the entrypoint for agreed architecture decisions. It records design agreement,
not a claim that the architecture is already implemented.

- Add or change decisions only after explicit agreement in the architecture discussion.
- Keep this document minimal and high-level.
- Do not promote proposals, examples, or implementation choices into agreed decisions.
- Concrete interfaces and internal implementation details remain undefined unless agreed.

## Guiding user-experience goal

**The framework must make non-functional tests easy to define through three explicit phases:
Declaration, Execution, and Evaluation.** This is the central design goal, not merely a description
of the implementation. Future interfaces and components should be judged against it.

The test author supplies the investigation-specific pieces; the core manages their lifecycle.
There is no implicit inference of collection needs from evaluation code.

### A. Declaration

The test inspects the supplied environment and declares what the run needs:

- Required topology or capabilities, such as at least two People collators.
- Additional node options, such as enabling a log target.
- Requested observations and any custom recorders.
- Selected reusable node checks and their associated observation setup.

Declaration can be a function receiving the proposed topology, rather than a large struct the
author must populate. It validates and requests configuration before execution; it does not modify
a running network. The core checks that the requests can be fulfilled.

### B. Execution

The author supplies workload generation, optional preparation such as proof generation or seeding,
and any custom recorder implementation the investigation needs.

The core coordinates the environment, workload, and recording lifecycle. Registered recorders run
during the relevant execution window and persist their observations.

Evidence can come from default node logs, additional logging enabled by requested node options,
and custom recorder output. A setting makes evidence available; recording preserves it. Recorder
output means recorded data, not necessarily an in-memory return value.

### C. Evaluation

The author defines three steps over the recorded evidence:

- **Extract:** build a check-specific view from logs and/or recorder records.
- **Validate:** judge that view and return a result with reasons.
- **Report:** explain the view and its validation result.

Reusable node checks and custom runtime checks follow the same pattern. Missing evidence must not
silently produce a passing result.

**Declaration says what must be available. Execution produces and preserves it.
Evaluation consumes and explains it.**

The exact query, recorder, and declaration APIs remain open. The phase boundaries and author/core
responsibilities are agreed.

## Current agreement

### Two layers

- **Shared core:** owns planning, execution, lifecycle, and common reporting.
- **Test-facing layer:** defines the investigation's declarations, workload preparation,
  custom recording, extraction, validation, and custom report content.

The test owns the meaning of the experiment; the core owns the mechanics of conducting it.

### Mostly declarative tests

Tests declare what they need and supply custom behavior where necessary. The core handles
coordination and lifecycle rather than requiring each test to reimplement them.

### High-level flow

```text
Declaration → Execution → Evaluation
Evaluation: extract → validate → report
```

Extraction and judgment are separate: `extract` builds a view of what happened;
`validate` determines whether it was acceptable.

### Preserved, reusable evidence

Evidence is preserved and shareable. Extraction, validation, and reporting can be rerun
without repeating execution.

### Incremental integration

Start with the existing PPN settings and workflow. Extend integration only when a concrete
need appears.

## Domain vocabulary and user goals

These concepts describe what the system should offer to test users and creators.
They do not prescribe separate software components, modules, or APIs.

### Test-user perspective

The user selects a **Test**, understands its **Target**, runs it in an
**Environment** with a **Load profile**, and receives a **Report** explaining
the result and its supporting evidence.

| Noun | Meaning |
| --- | --- |
| Test | A named, reusable investigation |
| Target | The component or behavior being investigated |
| Environment | The network, software versions, and settings used |
| Load profile | The intensity and timing of the applied workload |
| Run | One concrete execution of a test |
| Report | An explanation of the run, its assessment, and supporting evidence |

The Target is normally defined by the Test, not necessarily selected separately.

### Test-creator perspective

The creator defines the Test's declarations, workload preparation, any custom recording,
extraction, validation, and test-specific report content.

| Noun | Meaning |
| --- | --- |
| Requirements | The environment capabilities and evidence the Test needs |
| Workload | The operations and inputs prepared for execution |
| Evidence | Recorded facts from a Run |
| View | The check-specific representation extracted from Evidence |
| Measurement | Quantities derived from Evidence during extraction, where needed |
| Assessment | The result of validating a View, including the verdict, reasons, and relevant failure classifications |
| Report content | The test-specific explanation combined with the common report envelope |

Requirements belong to the Test. Workload describes **what** is applied;
Load profile describes **how much and when**. The extracted View describes **what
happened**; Assessment describes **what it means**.

These relationships establish the intended user experience and a shared vocabulary.
Their implementation remains open.
