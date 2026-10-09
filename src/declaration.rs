//! Phase A: describe the environment, sources, and workload required for a run.

use crate::execution::LoadGenerator;

trait Setup {
    type Load: LoadGenerator;

    async fn setup(
        self,
        context: &SetupContext,
    ) -> Result<Self::Load, SetupError>;
}


pub trait SourceSet {
    fn get(id: u32) -> String;
}

struct RunPlan<S, P: Setup> {
    environment: EnvironmentPlan,
    sources: S,
    setup: P,
    load_profile: LoadProfile,
}

pub trait Test {
    type Sources: SourceSet;

    type Load: LoadGenerator;

    fn declare(
        &self,
        topology: &NetworkTopology,
    ) -> Result<RunPlan<Self::Sources, Self::Load>, DeclarationError>;
}
