use helpers::{Report};

enum CheckResult {
    Pass,
    Warn(String),
    Fail(String),
    Missing(String),
}

enum ReportSection {
    TableRow
}


enum EvidenceError {
    Missing
}

trait QueryApi {
    fn query<T>(&self, query: &str) -> Result<T, EvidenceError>;
}


trait Check {
    type View;

    fn extract(
        &self,
        results: &dyn QueryApi,
    ) -> Result<Self::View, EvidenceError>;

    fn validate(
        &self,
        view: &Self::View,
    ) -> CheckResult;

    fn report(
        &self,
        view: &Self::View,
        result: &CheckResult,
    ) -> ReportSection;
}


/// ------- Try 2
///

/// ------- A. Declaration

trait LoadGenerator<Step, Load> {
    fn load_generation(step: Step) -> Load;
    fn get_load_plan() -> Steps;
}

trait SourceSet {
    fn get(id: u32) -> String; //tmp, until we figure this one out, but one of the types should and will be Recorder/Retriever as a Source
}

struct RunPlan<S, L: LoadGenerator> {
    sources: S,
    load_generator: L,
    load_profile: LoadProfile,
    // Resolved environment configuration and other run settings.
}

trait Test {
    type Sources: SourceSet;
    type Load: LoadGenerator;

    fn declare(
        &self,
        topology: &NetworkTopology,
    ) -> Result<
        RunPlan<Self::Sources, Self::Load>,
        DeclarationError,
    >;
}

/// ------- B. Execution

trait Retriever {
    type Record;

    fn retrieve(
        &self,
        results: &RecordedRun,
    ) -> Result<Vec<Self::Record>, EvidenceError>;
}

trait Recorder: Retriever {
    fn interval(&self) -> Duration;

    async fn record(
        &mut self,
        context: &RecordingContext,
    ) -> Result<Self::Record, RecordingError>;
}



trait Execution {
    async fn execute<S, L>(
        &self,
        plan: RunPlan<S, L>,
    ) -> Result<RunInput<S>, ExecutionError>
    where
        S: SourceSet,
        L: LoadGenerator;
}


/// ------- C. Evaluation

trait EvaluationAccess { //TODO: Need to access through our SourceSet
    fn node_logs(
        &self,
        node: &NodeId,
    ) -> Result<Vec<NodeLogRecord>, EvidenceError>;

    fn records<R: Retriever + 'static>(
        &self,
        source: &SourceId,
    ) -> Result<Vec<R::Record>, EvidenceError>;
}

trait Evaluation<Input: EvaluationAccess> {
    type View;

    fn extract(
        &self,
        input: &Input,
    ) -> Result<Self::View, EvidenceError>;

    fn validate(&self, view: &Self::View) -> CheckResult;

    fn report(
        &self,
        view: &Self::View,
        result: &CheckResult,
    ) -> ReportSection;
}
