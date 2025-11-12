# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 32 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 25 ms, 881 µs and 124 ns (76.35% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 539 ms, 559 µs and 446 ns | 10.80%     |         |
| SQL Workspace Generation     | 2 seconds, 599 ms, 989 µs and 384 ns | 7.93%      |         |
| Formatting Workspace         | 1 second, 610 ms, 826 µs and 871 ns  | 4.91%      |         |
| Checking Generated Workspace | 25 seconds, 25 ms, 881 µs and 124 ns | 76.35%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `buildable` which took 607 ms, 620 µs and 276 ns (23.37% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 8 ms, 195 µs and 276 ns   | 0.32%      |         |
| model                         | 305 ms, 82 µs and 584 ns  | 11.73%     |         |
| relations_trait               | 17 ms, 784 µs and 493 ns  | 0.68%      |         |
| attributes                    | 305 ms, 615 µs and 14 ns  | 11.75%     |         |
| value_settable_trait          | 6 ms, 91 µs and 808 ns    | 0.23%      |         |
| insertable_key_settable_trait | 10 ms, 820 µs and 378 ns  | 0.42%      |         |
| insertable                    | 524 ms, 557 µs and 878 ns | 20.18%     |         |
| buildable                     | 607 ms, 620 µs and 276 ns | 23.37%     |         |
| extension_attributes          | 176 ms, 233 µs and 165 ns | 6.78%      |         |
| workspace_write_to_disk       | 343 ms, 496 µs and 857 ns | 13.21%     |         |
| workspace_toml                | 294 ms, 160 µs and 774 ns | 11.31%     |         |
| workspace_rustfmt             | 330 µs and 881 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
