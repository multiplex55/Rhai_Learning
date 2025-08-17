# Testing

Run all tests:

```bash
cargo test
```

Each example that prints or uses `rhai::debug` writes its output to `logs/<example>.log`.  After running tests, view the log for an example:

```bash
cat logs/unit-tests.log
```
