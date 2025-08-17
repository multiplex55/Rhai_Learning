# Unit Test Style

Mimics unit testing with debug logs and assertions.

## Code

```rhai
debug("starting tests");
let x = 1 + 1;
assert(x == 2);
debug("math ok");
print(`x=${x}`);
true
```

## How It Works

`debug` writes diagnostic messages, while the Rust helper `assert` panics if
its condition is false. The script prints the final value of `x` and returns
`true` when all assertions pass.

Expected console output:

```
DEBUG: starting tests
DEBUG: math ok
x=2
```

## Key Points

- Combines `debug` and `assert` for lightweight test scripts.
- `assert` is registered in `src/examples/mod.rs`.

Note: The `debug` statements are captured by the host; see <https://rhai.rs/book/appendix/debugging.html>.
