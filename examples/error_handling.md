# Error Handling

Shows how to use `try`/`catch` and `throw` for custom error messages.

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
```
Running this prints `Caught: division by zero` and returns a map `{ msg: "division by zero", value: -1 }`.
