# Random Number

Rolls a six-sided die using a host function.

## Code

```rhai
let roll = rand_int(1, 6);
print(roll);
roll
```

## How It Works

`rand_int` is a Rust helper returning a random integer in the given range
(inclusive). The script prints the roll and returns it.

Expected console output: a number between `1` and `6`.

## Key Points

- Demonstrates calling host functions from Rhai.
- Results are nondeterministic.

Note: `rand_int` uses the host RNG; results cannot be reproduced without seeding.
