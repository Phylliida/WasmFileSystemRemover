Edit: Don't use this! It doesn't work. The problem is that having multiple memory is not supported so it's non-trivial to graft wasm fs into your wasm if you didn't compile with it. Also, extra bloat.

Instead what you should do is just instantiate both modules and use them that way.

# WasmFileSystemRemover
Replaces WASI file systems calls with calls to an in memory file system

This is a small modification of [https://github.com/wasm-forge/wasi2ic](https://github.com/wasm-forge/wasi2ic)'s code, which does a similar thing except for the internet computer.
