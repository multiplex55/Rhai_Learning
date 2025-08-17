# Basic Arithmetic and Control Flow

Adds numbers from 1 to 10 and reports the sum.

## Code

```rhai
let sum = 0;
for n in 1..=10 {
    sum += n;
}
if sum > 50 {
    print(`sum is ${sum}`);
} else {
    print("sum too small");
}
sum
```

## How It Works

The loop accumulates the numbers 1 through 10. A conditional prints whether
the total exceeds 50. The final expression evaluates to the sum.

Expected console output:

```
sum is 55
```

## Key Points

- Demonstrates `for` loops and `if` statements.
- Returns the computed `sum`.

Note: See <https://rhai.rs/book/control-flow.html> for more on Rhai control structures.
