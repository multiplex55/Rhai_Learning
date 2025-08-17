# Rhai Learning

An interactive desktop application for experimenting with the
[Rhai](https://rhai.rs) scripting language and the
[egui](https://docs.rs/egui) GUI framework.  The project bundles a set of
sample scripts and demonstrates how to embed a Rhai engine inside a Rust
application.

## Project Goals

* Explore Rhai syntax and features through runnable examples.
* Showcase integration patterns between Rhai and Rust/`egui`.
* Provide a playground for testing script hot‑swapping and data exchange
  techniques.

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository and run the examples:

   ```bash
   cargo run
   ```

The application window lists all available example scripts located under
[`examples/`](examples). Clicking **Run** executes the script and shows the
result in the console panel. Each example links to its documentation and
source so you can explore and modify the code.

## Examples

The project includes the following scripts:

- Hello World
- Basic Arithmetic
- Using a Rust Struct
- HTTP Request
- Serde Demo
- Performance Loop
- Unit Test Style
- Hot Swap
- Custom Module – load functions from a user-defined module.
- Async Simulation – mimic asynchronous tasks with a `sleep_ms` helper.

## UI Usage

* **Example List** – displayed on the left. Select an entry to view details.
* **Run** – executes the currently selected script.
* **Reload scripts** – reloads example files from disk, making it easy to test
  hot‑swapping.
* **Logs panel** – if an example produces a log file under `logs/`, the
  contents appear on the right.
* **Console panel** – shows printed output and the evaluation result.

Additional guides can be found in the [`docs/`](docs) directory or viewed as a
compiled book using [mdBook](https://rust-lang.github.io/mdBook/).

## Benchmarks

The project includes Criterion benchmarks that compare equivalent logic
implemented in pure Rust and in a Rhai script executed through the `Engine`.

Run the benchmarks with:

```bash
cargo bench
```

Benchmark reports are written to `target/criterion/`, where you can review
timing results and graphs for each benchmark.
