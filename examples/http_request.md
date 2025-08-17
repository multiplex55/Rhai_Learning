# HTTP Request

Fetches JSON from a remote API.

## Code

```rhai
let data = http_get("https://httpbin.org/get");
if type_of(data) == "map" {
    print(data.url);
    data.url
} else {
    print(data);
    data
}
```

## How It Works

`http_get` is a Rust helper using `reqwest` to perform a blocking HTTP GET and
deserialize the JSON response into a Rhai `map`. The script prints the `url`
field when successful; otherwise it prints an error string.

Expected console output:

```
https://httpbin.org/get
```

## Key Points

- Uses the `http_get` helper registered in `src/examples/mod.rs`.
- Demonstrates type checking with `type_of`.

Note: Network requests require the `reqwest` crate and internet access; see <https://rhai.rs/book/engine/https.html>.
