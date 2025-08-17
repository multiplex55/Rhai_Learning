# Performance Loop

Benchmarks a simple summation loop.

## Code

```rhai
let total = 0;
for n in 0..100000 {
    total += n;
}
print(total);
total
```

## How It Works

The loop adds integers from 0 to 99,999, prints the sum, and returns it. This
shows the cost of a tight loop in Rhai.

Expected console output:

```
4999950000
```

## Key Points

- Highlights looping performance.
- No Rust helpers are used; everything runs in Rhai.

Note: Use release builds for accurate benchmarks; see <https://rhai.rs/book/performance/benchmarking.html>.
