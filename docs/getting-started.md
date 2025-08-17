# Getting Started with Rhai

Rhai is a small and friendly scripting language for Rust. A script is a plain
text file containing statements and expressions. Basic arithmetic and variable
binding look like this:

```rhai
let x = 40 + 2;
print(x);
```

You can run the above snippet in the app via the
[basic arithmetic example](../examples/basic_arith.rhai). For a full tour of
the language syntax see the [official Rhai book](https://rhai.rs/book/).

### Integrating Rhai

The simplest way to embed Rhai is to create an [`Engine`](https://docs.rs/rhai)
and call `eval` on your script:

```rust
let engine = rhai::Engine::new();
let result: i64 = engine.eval("1 + 2 + 3")?;
```

This project demonstrates a richer integration where the `Engine` is wired into
an [`egui`](https://docs.rs/egui) user interface. Explore the rest of the docs
for more advanced patterns.

