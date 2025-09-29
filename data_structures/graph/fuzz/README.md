# Graph fuzzing

Honggfuzz-based fuzzing for graph data structures.

If you have not installed honggfuzz, run `cargo install honggfuzz`.

## Harnesses

### RootNodes

```bash
cargo hfuzz run root_nodes
```

and to run the crash cases:

```bash
cargo hfuzz run-debug root_nodes hfuzz_workspace/*/*.fuzz
```

### SinkNodes

```bash
cargo hfuzz run sink_nodes
```

and to run the crash cases:

```bash
cargo hfuzz run-debug sink_nodes hfuzz_workspace/*/*.fuzz
```

### SimplePath

```bash
cargo hfuzz run sink_nodes
```

and to run the crash cases:

```bash
cargo hfuzz run-debug sink_nodes hfuzz_workspace/*/*.fuzz
```

### Lin

```bash
cargo hfuzz run lin
```

and to run the crash cases:

```bash
cargo hfuzz run-debug lin hfuzz_workspace/*/*.fuzz
```
