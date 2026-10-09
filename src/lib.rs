//! Draft non-functional testing interfaces organized by phase.
//!
//! Supporting types and signatures remain unfinished. Experiments outside the
//! current candidate interfaces live in `scratch/` and are not library modules.

pub mod declaration;
pub mod evaluation;
pub mod execution;

pub use declaration::{RunPlan, SourceSet, Test};
pub use evaluation::{
    CheckResult, Evaluation, EvaluationAccess, NodeId, NodeLogRecord, Report, ReportSection,
};
pub use execution::{EvidenceError, Execution, LoadGenerator, Recorder, Retriever};
