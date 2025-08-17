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

#[test]
fn collections_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "collections")
        .expect("collections example");
    let result = ex.run();
    assert_eq!(result.stdout, "");
    assert_eq!(result.value.clone_cast::<i64>(), 12);
}

#[test]
fn error_handling_example_catches() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "error-handling")
        .expect("error-handling example");
    let result = ex.run();
    let map = result.value.clone_cast::<rhai::Map>();
    assert_eq!(map["msg"].clone_cast::<String>(), "division by zero");
    assert_eq!(map["value"].clone_cast::<i64>(), -1);
}

#[test]
fn random_example_rolls_die() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "random")
        .expect("random example");
    let result = ex.run();
    let roll: i64 = result.stdout.trim().parse().expect("number");
    assert!(roll >= 1 && roll <= 6);
    assert_eq!(roll, result.value.clone_cast::<i64>());
}
