# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 32 seconds.
The slowest task was `Checking Generated Workspace` which took 22 seconds, 23 ms, 362 µs and 982 ns (68.75% of all time).

| name                                        | time                                 | percentage | comment |
|---------------------------------------------|--------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 444 ms, 491 µs and 174 ns | 9.38%      |         |
| SQL Workspace Generation                    | 5 seconds, 759 ms, 417 µs and 735 ns | 15.62%     |         |
| Formatting and Checking Generated Workspace | 1 second, 625 ms, 50 µs and 390 ns   | 3.12%      |         |
| Checking Generated Workspace                | 22 seconds, 23 ms, 362 µs and 982 ns | 68.75%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 2 seconds, 962 ms, 834 µs and 544 ns (40.00% of all time).

| name                    | time                                 | percentage | comment |
|-------------------------|--------------------------------------|------------|---------|
| schema_macro            | 20 ms, 951 µs and 962 ns             | 0.00%      |         |
| model                   | 92 ms, 406 µs and 263 ns             | 0.00%      |         |
| relations_trait         | 26 ms, 290 µs and 375 ns             | 0.00%      |         |
| attributes              | 33 ms, 526 µs and 584 ns             | 0.00%      |         |
| value_settable_trait    | 10 ms, 414 µs and 279 ns             | 0.00%      |         |
| insertable              | 23 ms, 363 µs and 327 ns             | 0.00%      |         |
| buildable               | 58 ms, 545 µs and 742 ns             | 0.00%      |         |
| extension_attributes    | 22 ms, 395 µs and 898 ns             | 0.00%      |         |
| workspace_write_to_disk | 2 seconds, 508 ms, 441 µs and 637 ns | 40.00%     |         |
| workspace_toml          | 2 seconds, 962 ms, 834 µs and 544 ns | 40.00%     |         |
| workspace_rustfmt       | 247 µs and 124 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
