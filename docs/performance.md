# Performance Considerations and Benchmarks

Rhai is designed to be lightweight, but there are a few tips to keep your
scripts snappy:

* Compile scripts once and reuse the [`AST`](https://rhai.rs/book/advanced/ast.html).
* Avoid excessive dynamic allocations inside tight loops.
* When calling Rust functions, prefer small `Copy` types over large `Struct`s.

The [performance loop example](../examples/perf_loop.rhai) measures how quickly
Rhai can execute a simple counted loop. Use it as a starting point for your own
benchmarks.

See the [Rhai performance guide](https://rhai.rs/book/performance/index.html)
for more techniques and discussion.

