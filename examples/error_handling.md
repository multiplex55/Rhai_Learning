# Error Handling

Shows using `throw` with `try`/`catch`.

## Code

```rhai
fn divide(x, y) {
    if y == 0 {
        throw "division by zero";
    }
    x / y
}

let message = "";
let value = 0;
try {
    value = divide(10, 0);
} catch (err) {
    message = err;
    value = -1;
}
print("Caught: " + message);

#{ msg: message, value: value }
```

## How It Works

`divide` throws an error when dividing by zero. The `try` block catches the
error, records the message, and sets a fallback value. The script prints the
captured message and returns it together with the value in a map.

Expected console output:

```
Caught: division by zero
```

## Key Points

- Illustrates `throw`, `try`, and `catch`.
- Returns a map with the error message and computed value.

Note: Error strings can be any `Dynamic`; see <https://rhai.rs/book/control-flow/error.html>.
