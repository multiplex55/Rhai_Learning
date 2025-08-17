# Serde Demo

Serializes and deserializes values with JSON.

## Code

```rhai
let data = #{ name: "Alice", age: 30 };
let json = to_json(data);
print(json);
let parsed = from_json(json);
print(`${parsed.name} is ${parsed.age} years old`);
parsed
```

## How It Works

`to_json` and `from_json` are Rust helpers backed by `serde_json`. The map is
serialized to a JSON string, printed, parsed back, and printed again. The
deserialized map is returned.

Expected console output:

```
{"name":"Alice","age":30}
Alice is 30 years old
```

## Key Points

- Shows round-tripping data between Rhai and JSON.
- Uses the `serde` feature via `to_json`/`from_json`.

Note: Requires enabling `serde` support in the host application; see <https://rhai.rs/book/features/serde.html>.
