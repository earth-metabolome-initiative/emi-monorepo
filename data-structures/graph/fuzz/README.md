# Graph fuzzing

Honggfuzz-based fuzzing for graph data structures.

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