# Custom Module

Loads a user-defined module and calls its function.

## Code

```rhai
import "math_utils.rhai" as math;

let n = 4;
let sq = math::square(n);
print("square(" + n.to_string() + ") = " + sq.to_string());
sq
```

## How It Works

`import` brings in `math_utils.rhai` as `math`. The `square` function is
called to compute `n * n`, the result is printed, and the value `sq` is
returned.

Expected console output:

```
square(4) = 16
```

## Key Points

- Demonstrates module loading with `import`.
- No Rust helpers are used.

Note: Module paths are relative to the script; see <https://rhai.rs/book/modules/import.html>.
