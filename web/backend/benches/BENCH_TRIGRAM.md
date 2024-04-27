# Benchmarking Trigram
Comparing the performance of trigram similarity between [ngrammatics's implementation BiWebgraph, Bit Field Vector](https://github.com/LucaCappelletti94/ngrammatic), and [Postgres pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html).

## How to run the benchmark

To run the benchmark, you can run:

```bash
RUSTFLAGS="-C target-cpu=native" cargo bench --bench bench_trigram
```

## Benchmarks on Open tree of life taxonomy
In this benchmark, we ran queries on the Open tree of life taxonomy. This includes about 4.5M entries. We compared the performance of trigram similarity between BiWebgraph, Bit Field Vector, and Postgres pg_trgm. We ran the benchmark on the lowercased and non-lowercased version of the taxonomy. We ran the benchmark in sequential and parallel mode.

*Compared to the benchmark on NCBI, here we are using the Postgres operators instead of the similarity function. This is because the similarity function does not actually employ the trigram index. The operators do use the trigram index.*

We have not yet measured the different memory usage of the data structures. We will add this information in the future.

| Lowercased | Data Structure           | Type       | Time 1      | Time 2      | Time 3      |
|------------|--------------------------|------------|-------------|-------------|-------------|
| Yes        | BiWebgraph               | Sequential | 4.7489 s    | 4.7495 s    | 4.7501 s    |
| No         | Bit Field Vector         | Sequential | 832.56 ms   | 832.71 ms   | 832.85 ms   |
| Yes        | Postgres (% & <->)       | Sequential | 309.57 ms   | 309.60 ms   | 309.62 ms   |
| Yes        | BiWebgraph               | Parallel   | 1.4514 s    | 1.4538 s    | 1.4564 s    |
| No         | Bit Field Vector         | Parallel   | 286.36 ms   | 287.90 ms   | 289.43 ms   |
| Yes        | BiWebgraph               | Sequential | 5.4268 s    | 5.4274 s    | 5.4279 s    |
| No         | Bit Field Vector         | Sequential | 933.60 ms   | 933.96 ms   | 934.34 ms   |
| Yes        | BiWebgraph               | Parallel   | 1.4600 s    | 1.4625 s    | 1.4651 s    |
| No         | Bit Field Vector         | Parallel   | 279.95 ms   | 281.32 ms   | 282.69 ms   |


## Benchmarks on NCBI taxonomy
In this benchmark, we ran queries on the NCBI taxonomy. This includes about 2.5M entries. We compared the performance of trigram similarity between BiWebgraph, Bit Field Vector, and Postgres pg_trgm. We ran the benchmark on the lowercased and non-lowercased version of the taxonomy. We ran the benchmark in sequential and parallel mode.

*Compared to the benchmark on Open tree of life, here we are using the similarity function of Postgres. This was done because, when we ran this benchmark, we were not aware that the similarity function does not actually employ the trigram index.*

We have not yet measured the different memory usage of the data structures. We will add this information in the future.

| Lowercased | Data Structure                         | Type      | Time 1            | Time 2            | Time 3            |
|------------|----------------------------------------|-----------|-------------------|-------------------|-------------------|
| No         | BiWebgraph                             | Sequential| 1.9611 s          | 1.9614 s          | 1.9618 s          |
| No         | Bit Field Vector                       | Sequential| 359.50 ms         | 359.61 ms         | 359.73 ms         |
| Yes        | Postgres (similarity)                  | Sequential| 5.0970 s          | 5.0978 s          | 5.0986 s          |
| No         | BiWebgraph                             | Parallel  | 609.99 ms         | 611.52 ms         | 613.06 ms         |
| No         | Bit Field Vector                       | Parallel  | 161.96 ms         | 163.09 ms         | 164.22 ms         |
| Yes        | BiWebgraph                             | Sequential| 2.2726 s          | 2.2738 s          | 2.2751 s          |
| No         | Bit Field Vector                       | Sequential| 403.07 ms         | 403.32 ms         | 403.59 ms         |
| Yes        | BiWebgraph                             | Parallel  | 619.18 ms         | 620.55 ms         | 621.99 ms         |
| No         | Bit Field Vector                       | Parallel  | 153.59 ms         | 154.38 ms         | 155.18 ms         |
