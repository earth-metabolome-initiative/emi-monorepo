# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 37 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 795 ms, 737 µs and 326 ns (67.57% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 373 ms, 883 µs and 975 ns  | 8.11%      |         |
| SQL Workspace Generation     | 6 seconds, 958 ms, 954 µs and 505 ns  | 16.22%     |         |
| Formatting Workspace         | 1 second, 619 ms, 778 µs and 463 ns   | 2.70%      |         |
| Checking Generated Workspace | 25 seconds, 795 ms, 737 µs and 326 ns | 67.57%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 500 ms, 600 µs and 291 ns (33.33% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 8 ms, 582 µs and 45 ns               | 0.00%      |         |
| model                         | 318 ms, 690 µs and 854 ns            | 0.00%      |         |
| relations_trait               | 17 ms, 920 µs and 5 ns               | 0.00%      |         |
| attributes                    | 303 ms, 853 µs and 707 ns            | 0.00%      |         |
| value_settable_trait          | 6 ms, 241 µs and 716 ns              | 0.00%      |         |
| insertable_key_settable_trait | 11 ms, 10 µs and 320 ns              | 0.00%      |         |
| insertable                    | 527 ms, 170 µs and 129 ns            | 0.00%      |         |
| buildable                     | 613 ms, 52 µs and 963 ns             | 0.00%      |         |
| extension_attributes          | 175 ms, 288 µs and 11 ns             | 0.00%      |         |
| workspace_write_to_disk       | 2 seconds, 500 ms, 600 µs and 291 ns | 33.33%     |         |
| workspace_toml                | 2 seconds, 476 ms, 230 µs and 418 ns | 33.33%     |         |
| workspace_rustfmt             | 314 µs and 46 ns                     | 0.00%      |         |

![Plot](emi_codegen.png)
