# Using Rust Types and External Crates from Rhai

Rhai can call functions and methods defined in Rust. To expose a custom type,
register it with the scripting engine and provide constructor or method
bindings. The [struct usage example](../examples/use_struct.rhai) showcases a
`Point` type defined in Rust:

```rust
engine.register_type::<Point>();
engine.register_fn("Point", Point::new);
engine.register_fn("length", Point::length);
```

External crates can be wrapped in a similar fashion. The
[HTTP request example](../examples/http_request.rhai) exposes a helper that
uses the [`reqwest`](https://docs.rs/reqwest) crate to fetch JSON data over the
network.

For deeper integration techniques consult the
[Rhai embedding guide](https://rhai.rs/book/engine/customize.html).

