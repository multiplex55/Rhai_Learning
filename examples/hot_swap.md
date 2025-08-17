# Hot Swap

Reads a message from an external file.

## Code

```rhai
let msg = read_file("examples/hot_message.txt");
print(msg);
msg
```

## How It Works

`read_file` is a Rust helper that loads the text from `hot_message.txt`. The
value is printed and then returned. Editing the file and rerunning the script
changes the output.

Expected console output:

```
Initial message
```

## Key Points

- Demonstrates using the `read_file` helper to read disk files.
- Mimics hot-swapping by reloading external content.

Note: `read_file` reads relative to the process working directory.
