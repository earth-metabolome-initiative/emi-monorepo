# Benchmarking Trigram
In this benchmark, we compare the performance of the data structures provided by ngrammatic or the PostgreSQL database.

| Lowercased | Data Structure                         | Type      | Time 1            | Time 2            | Time 3            |
|------------|----------------------------------------|-----------|-------------------|-------------------|-------------------|
| No         | BiWebgraph                             | Sequential| 1.9611 s          | 1.9614 s          | 1.9618 s          |
| No         | Bit Field Bipartite Graph              | Sequential| 359.50 ms         | 359.61 ms         | 359.73 ms         |
| -          | Postgres                               | -         | 5.0970 s          | 5.0978 s          | 5.0986 s          |
| No         | BiWebgraph                             | Parallel  | 609.99 ms         | 611.52 ms         | 613.06 ms         |
| No         | Bit Field Bipartite Graph              | Parallel  | 161.96 ms         | 163.09 ms         | 164.22 ms         |
| Yes        | BiWebgraph                             | Sequential| 2.2726 s          | 2.2738 s          | 2.2751 s          |
| No         | Bit Field Bipartite Graph             | Sequential| 403.07 ms         | 403.32 ms         | 403.59 ms         |
| Yes        | BiWebgraph                             | Parallel  | 619.18 ms         | 620.55 ms         | 621.99 ms         |
| No         | Bit Field Bipartite Graph             | Parallel  | 153.59 ms         | 154.38 ms         | 155.18 ms         |
