let plan = RunPlan::new(topology, load_generator, load_profile)
    .record::<PeopleRings, _>(ring_recorder)
    .record::<PeopleMetrics, _>(metrics_recorder);
