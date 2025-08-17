# Using a Rust Struct

Calls methods on a `Point` type defined in Rust.

## Code

```rhai
let p = Point(3, 4);
print(`length: ${p.length()}`);
p.length()
```

## How It Works

The `Point` struct and its `length` method are registered from Rust. The script
constructs a point, prints its length, and returns the numeric result.

Expected console output:

```
length: 5
```

## Key Points

- Demonstrates binding a Rust type into Rhai.
- Uses `Point` and `length` registered in `src/examples/mod.rs`.

Note: Struct methods mutate `self` by default unless declared otherwise.
