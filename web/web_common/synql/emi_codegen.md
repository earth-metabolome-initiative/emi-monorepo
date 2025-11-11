# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 33 seconds.
The slowest task was `Checking Generated Workspace` which took 23 seconds, 537 ms, 244 µs and 896 ns (69.70% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 4 seconds, 250 ms, 128 µs and 360 ns  | 12.12%     |         |
| SQL Workspace Generation                    | 4 seconds, 668 ms, 802 µs and 391 ns  | 12.12%     |         |
| Formatting and Checking Generated Workspace | 1 second, 274 ms, 246 µs and 830 ns   | 3.03%      |         |
| Checking Generated Workspace                | 23 seconds, 537 ms, 244 µs and 896 ns | 69.70%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 541 ms, 689 µs and 217 ns (25.00% of all time).

| name                    | time                                | percentage | comment |
|-------------------------|-------------------------------------|------------|---------|
| schema_macro            | 11 ms, 215 µs and 307 ns            | 0.00%      |         |
| model                   | 368 ms, 521 µs and 38 ns            | 0.00%      |         |
| relations_trait         | 23 ms, 41 µs and 78 ns              | 0.00%      |         |
| attributes              | 344 ms, 924 µs and 348 ns           | 0.00%      |         |
| value_settable_trait    | 7 ms, 734 µs and 113 ns             | 0.00%      |         |
| insertable              | 366 ms, 357 µs and 448 ns           | 0.00%      |         |
| buildable               | 701 ms, 745 µs and 750 ns           | 0.00%      |         |
| extension_attributes    | 199 ms, 140 µs and 949 ns           | 0.00%      |         |
| workspace_write_to_disk | 1 second, 541 ms, 689 µs and 217 ns | 25.00%     |         |
| workspace_toml          | 1 second, 104 ms, 128 µs and 762 ns | 25.00%     |         |
| workspace_rustfmt       | 304 µs and 381 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
