// Uncompiled API sketch: registering recorders grows the run's typed source set.
// These identifiers and builder methods are not implemented yet.

let plan = RunPlan::new(topology, load_generator, load_profile)
    .record::<PeopleRings, _>(ring_recorder)
    .record::<PeopleMetrics, _>(metrics_recorder);
