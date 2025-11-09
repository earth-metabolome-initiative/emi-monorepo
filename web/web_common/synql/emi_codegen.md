# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 29 seconds.
The slowest task was `Checking Generated Workspace` which took 22 seconds, 385 ms, 601 µs and 477 ns (75.86% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 599 ms, 848 µs and 775 ns  | 10.34%     |         |
| SQL Workspace Generation                    | 2 seconds, 626 ms, 820 µs and 893 ns  | 6.90%      |         |
| Formatting and Checking Generated Workspace | 1 second, 349 ms, 118 µs and 885 ns   | 3.45%      |         |
| Checking Generated Workspace                | 22 seconds, 385 ms, 601 µs and 477 ns | 75.86%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 1 second, 535 ms, 513 µs and 685 ns (50.00% of all time).

| name                    | time                                | percentage | comment |
|-------------------------|-------------------------------------|------------|---------|
| schema_macro            | 10 ms, 896 µs and 434 ns            | 0.00%      |         |
| model                   | 79 ms, 608 µs and 854 ns            | 0.00%      |         |
| relations_trait         | 11 ms, 988 µs and 207 ns            | 0.00%      |         |
| attributes              | 17 ms, 56 µs and 451 ns             | 0.00%      |         |
| value_settable_trait    | 5 ms, 252 µs and 378 ns             | 0.00%      |         |
| insertable              | 11 ms, 542 µs and 187 ns            | 0.00%      |         |
| buildable               | 28 ms, 213 µs and 349 ns            | 0.00%      |         |
| extension_attributes    | 11 ms, 170 µs and 911 ns            | 0.00%      |         |
| workspace_write_to_disk | 915 ms, 270 µs and 401 ns           | 0.00%      |         |
| workspace_toml          | 1 second, 535 ms, 513 µs and 685 ns | 50.00%     |         |
| workspace_rustfmt       | 308 µs and 36 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
