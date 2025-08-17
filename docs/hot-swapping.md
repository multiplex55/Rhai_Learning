# Hot Swapping and Runtime Code Reloading

One of Rhai's strengths is the ability to reload scripts without restarting
the host application. The [hot swap example](../examples/hot_swap.rhai) works
with a text file and demonstrates updating logic while the UI stays running.

The application exposes a **Reload scripts** button which refreshes the example
manifest and any modified files. This allows rapid iteration on game logic or
configuration scripts.

More strategies for hot reloading are covered in the
[Rhai book on dynamic modules](https://rhai.rs/book/engine/modules/dynamic.html).

