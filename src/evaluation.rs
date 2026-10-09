//! Phase C: extract a view, validate it, and explain the result.

use crate::execution::{EvidenceError, Retriever};

pub type NodeId = u32;

pub type NodeLogRecord = String;

pub enum CheckResult {
    Pass,
    Warn(String),
    Fail(String),
    Missing(String),
}

pub enum Report {}

pub enum ReportSection {
    TableRow,
}

/// TODO: access recorded sources through the declared, typed source set.
pub trait EvaluationAccess {
    fn node_logs(&self, node: &NodeId) -> Result<Vec<NodeLogRecord>, EvidenceError>;

    /// Temporary source lookup while typed access is being designed.
    fn records<R: Retriever + 'static>(
        &self,
        source: &SourceId,
    ) -> Result<Vec<R::Record>, EvidenceError>;
}

pub trait Evaluation<Input: EvaluationAccess> {
    type View;

    fn extract(&self, input: &Input) -> Result<Self::View, EvidenceError>;
    fn validate(&self, view: &Self::View) -> CheckResult;
    fn report(&self, view: &Self::View, result: &CheckResult) -> ReportSection;
}
