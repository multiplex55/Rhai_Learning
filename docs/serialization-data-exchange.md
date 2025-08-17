# Serialization and Data Exchange

Scripts often need to talk to the outside world. Using
[Serde](https://serde.rs/) together with Rhai's dynamic type system makes it
easy to send and receive structured data. The
[serialization example](../examples/serde_demo.rhai) demonstrates converting
between Rhai `Dynamic` values and JSON strings:

```rust
engine.register_fn("to_json", to_json);
engine.register_fn("from_json", from_json);
```

These helpers use `serde_json` under the hood to serialize/deserialize values.
For a deeper discussion see the
[Rhai book chapter on serialization](https://rhai.rs/book/engine/serialization.html).

