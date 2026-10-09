//! Phase B: generate workload and produce recorded observations.
//!
//! Recording and retrieval stay together as one production/readback contract.

use std::time::Duration;

use crate::declaration::{RunPlan, SourceSet};

pub enum EvidenceError {
    Missing,
}

pub trait LoadGenerator<Step, Load> {
    fn load_generation(step: Step) -> Load;
    fn get_load_plan() -> Steps;
}

pub trait Retriever {
    type Record;
    fn retrieve(&self, results: &RecordedRun) -> Result<Vec<Self::Record>, EvidenceError>;
}

pub trait Recorder: Retriever {
    fn interval(&self) -> Duration;
    async fn record(&mut self, context: &RecordingContext) -> Result<Self::Record, RecordingError>;
}

pub trait Execution {
    async fn execute<S, L>(&self, plan: RunPlan<S, L>) -> Result<RunInput<S>, ExecutionError>
    where
        S: SourceSet,
        L: LoadGenerator;
}
