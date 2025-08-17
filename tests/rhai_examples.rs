use Rhai_Learning::examples::ExampleRegistry;

#[test]
fn example_docs_exist() {
    let registry = ExampleRegistry::all();
    for ex in &registry {
        assert!(ex.doc_html_path.exists(), "missing doc for {}", ex.id);
    }
}

#[test]
fn hello_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "hello")
        .expect("hello example");
    let value = ex.run().expect("script run");
    assert_eq!(value.clone_cast::<String>(), "hello from rhai");
}

#[test]
fn unit_test_example_logs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "unit-tests")
        .expect("unit-tests example");
    let value = ex.run().expect("script run");
    assert!(value.as_bool().unwrap());
    let expected = "DEBUG: \"starting tests\"\nDEBUG: \"math ok\"\nx=2\n";
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
    let value = ex.run().expect("script run");
    let log = std::fs::read_to_string("logs/custom-module.log").expect("log file");
    assert!(log.contains("square(4) = 16"));
    assert_eq!(value.clone_cast::<i64>(), 16);
}

#[test]
fn async_sim_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "async-sim")
        .expect("async-sim example");
    let value = ex.run().expect("script run");
    let log = std::fs::read_to_string("logs/async-sim.log").expect("log file");
    assert!(log.contains("task complete"));
    assert_eq!(value.clone_cast::<String>(), "done");
}

#[test]
fn collections_example_runs() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "collections")
        .expect("collections example");
    let value = ex.run().expect("script run");
    assert_eq!(value.clone_cast::<i64>(), 12);
}

#[test]
fn error_handling_example_catches() {
    let registry = ExampleRegistry::all();
    let ex = registry
        .iter()
        .find(|e| e.id == "error-handling")
        .expect("error-handling example");
    let value = ex.run().expect("script run");
    let map = value.clone_cast::<rhai::Map>();
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
    let value = ex.run().expect("script run");
    let log = std::fs::read_to_string("logs/random.log").expect("log file");
    let roll: i64 = log.trim().parse().expect("number");
    assert!(roll >= 1 && roll <= 6);
    assert_eq!(roll, value.clone_cast::<i64>());
}
