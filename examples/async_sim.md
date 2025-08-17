# Async Simulation

Simulates a short delay by pausing execution.

## Code

```rhai
print("starting task...");
sleep_ms(50);
print("task complete");
"done"
```

## How It Works

`sleep_ms` is a Rust helper that blocks the current thread for the given
milliseconds. The script prints messages before and after the pause and
returns "done".

Expected console output:

```
starting task...
task complete
```

## Key Points

- Uses the Rust function `sleep_ms` registered in `src/examples/mod.rs`.
- Demonstrates sequencing of work around delays.

Note: This helper merely sleeps the thread; it is not asynchronous. See <https://rhai.rs/book/operations/sleep.html>.
