# Collections & Iteration

Mutates an array and a map before computing a total.

## Code

```rhai
let numbers = [1, 2, 3];
for i in 0..numbers.len() {
    numbers[i] *= 2;
}
let total = 0;
for n in numbers {
    total += n;
}
let scores = #{ "Alice": 1, "Bob": 2 };
for name in scores.keys() {
    scores[name] += 1;
}
scores["Cara"] = total;
numbers.push(scores["Bob"]);

total
```

## How It Works

The script doubles each element of `numbers`, sums the results into `total`,
and updates a `scores` map. The last expression yields the total sum.

Expected console output: *(none)*

## Key Points

- Shows array indexing, iteration and mutation.
- Demonstrates map operations and dynamic key access.
- Returns the final `total` (12).

Note: Collections in Rhai are dynamically typed; see <https://rhai.rs/book/language/arrays-maps.html>.
