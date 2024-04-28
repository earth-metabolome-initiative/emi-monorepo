# Benchmarking Trigram
Comparing the performance of trigram similarity between [ngrammatics's implementation BiWebgraph, Bit Field Vector](https://github.com/LucaCappelletti94/ngrammatic), and [Postgres pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html).

## How to run the benchmark

To run the benchmark, you can run:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --bench bench_trigram
```

## Benchmarks on Open tree of life taxonomy 
In this benchmark, we ran queries on the Open tree of life taxonomy. This includes about 4.5M entries. We compared the performance of trigram similarity between BiWebgraph, Bit Field Vector, and Postgres pg_trgm. We ran the benchmark on the lowercased and non-lowercased version of the taxonomy. We ran the benchmark in sequential and parallel mode.

We have not yet measured the different memory usage of the data structures. We will add this information in the future.

| Lowercased | Data Structure                  | Index Type | Type       | Time 1      | Time 2      | Time 3      |
|------------|---------------------------------|------------|------------|-------------|-------------|-------------|
| Yes        | postgres similarity             | GIST       | -          | 3.0437 s    | 3.0453 s    | 3.0470 s    |
| Yes        | postgres word similarity        | GIST       | -          | 2.1236 s    | 2.1250 s    | 2.1266 s    |
| Yes        | postgres strict word similarity | GIST       | -          | 2.1334 s    | 2.1350 s    | 2.1367 s    |
| Yes        | postgres similarity             | GIN        | -          | 308.55 ms   | 308.67 ms   | 308.79 ms   |
| Yes        | postgres word similarity        | GIN        | -          | 111.33 ms   | 111.34 ms   | 111.34 ms   |
| Yes        | postgres strict word similarity | GIN        | -          | 147.87 ms   | 147.88 ms   | 147.90 ms   |
| No         | BiWebgraph                      | -          | Sequential | 4.7489 s    | 4.7495 s    | 4.7501 s    |
| No         | Bit Field Vector                | -          | Sequential | 832.56 ms   | 832.71 ms   | 832.85 ms   |
| No         | BiWebgraph                      | -          | Parallel   | 1.4514 s    | 1.4538 s    | 1.4564 s    |
| No         | Bit Field Vector                | -          | Parallel   | 286.36 ms   | 287.90 ms   | 289.43 ms   |
| Yes        | BiWebgraph                      | -          | Sequential | 5.4268 s    | 5.4274 s    | 5.4279 s    |
| Yes        | Bit Field Vector                | -          | Sequential | 933.60 ms   | 933.96 ms   | 934.34 ms   |
| Yes        | BiWebgraph                      | -          | Parallel   | 1.4600 s    | 1.4625 s    | 1.4651 s    |
| Yes        | Bit Field Vector                | -          | Parallel   | 279.95 ms   | 281.32 ms   | 282.69 ms   |
