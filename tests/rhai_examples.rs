use Rhai_Learning::examples::ExampleRegistry;

#[test]
fn hello_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "hello")
        .expect("hello example");
    let result = ex.run();
    assert_eq!(result.stdout, "");
    assert_eq!(result.value.clone_cast::<String>(), "hello from rhai");
}

#[test]
fn unit_test_example_logs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "unit-tests")
        .expect("unit-tests example");
    let result = ex.run();
    let expected = "DEBUG: \"starting tests\"\nDEBUG: \"math ok\"\nx=2\n";
    assert_eq!(result.stdout, expected);
    assert_eq!(result.value.as_bool().unwrap(), true);
    let log = std::fs::read_to_string("logs/unit-tests.log").expect("log file");
    assert_eq!(log, expected);
}

#[test]
fn custom_module_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "custom-module")
        .expect("custom-module example");
    let result = ex.run();
    assert!(result.stdout.contains("square(4) = 16"));
    assert_eq!(result.value.clone_cast::<i64>(), 16);
}

#[test]
fn async_sim_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "async-sim")
        .expect("async-sim example");
    let result = ex.run();
    assert!(result.stdout.contains("task complete"));
    assert_eq!(result.value.clone_cast::<String>(), "done");
}
