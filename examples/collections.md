# Collections & Iteration

Demonstrates creating arrays and maps, iterating over them, and mutating values.

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
```

The final value `total` is returned, showing the result of these mutations.
