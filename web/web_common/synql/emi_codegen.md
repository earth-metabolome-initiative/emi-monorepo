# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 33 seconds.
The slowest task was `Checking Generated Workspace` which took 24 seconds, 197 ms, 723 µs and 341 ns (72.73% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 849 ms, 672 µs and 695 ns  | 9.09%      |         |
| SQL Workspace Generation                    | 4 seconds, 159 ms, 767 µs and 540 ns  | 12.12%     |         |
| Formatting and Checking Generated Workspace | 1 second, 421 ms, 702 µs and 207 ns   | 3.03%      |         |
| Checking Generated Workspace                | 24 seconds, 197 ms, 723 µs and 341 ns | 72.73%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 306 ms, 231 µs and 390 ns (50.00% of all time).

| name                    | time                                 | percentage | comment |
|-------------------------|--------------------------------------|------------|---------|
| schema_macro            | 11 ms, 550 µs and 872 ns             | 0.00%      |         |
| model                   | 84 ms, 741 µs and 294 ns             | 0.00%      |         |
| relations_trait         | 12 ms, 590 µs and 377 ns             | 0.00%      |         |
| attributes              | 17 ms, 743 µs and 314 ns             | 0.00%      |         |
| value_settable_trait    | 5 ms, 381 µs and 177 ns              | 0.00%      |         |
| insertable              | 12 ms, 738 µs and 173 ns             | 0.00%      |         |
| buildable               | 29 ms, 321 µs and 623 ns             | 0.00%      |         |
| extension_attributes    | 11 ms, 405 µs and 330 ns             | 0.00%      |         |
| workspace_write_to_disk | 2 seconds, 306 ms, 231 µs and 390 ns | 50.00%     |         |
| workspace_toml          | 1 second, 667 ms, 731 µs and 756 ns  | 25.00%     |         |
| workspace_rustfmt       | 332 µs and 234 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
