# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 30 seconds.
The slowest task was `Checking Generated Workspace` which took 19 seconds, 973 ms, 210 µs and 169 ns (63.33% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 377 ms, 533 µs and 634 ns  | 10.00%     |         |
| SQL Workspace Generation                    | 5 seconds, 443 ms, 924 µs and 378 ns  | 16.67%     |         |
| Formatting and Checking Generated Workspace | 1 second, 328 ms, 296 µs and 331 ns   | 3.33%      |         |
| Checking Generated Workspace                | 19 seconds, 973 ms, 210 µs and 169 ns | 63.33%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 124 ms, 663 µs and 674 ns (40.00% of all time).

| name                    | time                                 | percentage | comment |
|-------------------------|--------------------------------------|------------|---------|
| schema_macro            | 12 ms, 720 µs and 832 ns             | 0.00%      |         |
| model                   | 293 ms, 370 µs and 675 ns            | 0.00%      |         |
| relations_trait         | 17 ms, 82 µs and 102 ns              | 0.00%      |         |
| attributes              | 287 ms, 455 µs and 490 ns            | 0.00%      |         |
| value_settable_trait    | 5 ms, 898 µs and 720 ns              | 0.00%      |         |
| insertable              | 301 ms, 268 µs and 739 ns            | 0.00%      |         |
| buildable               | 571 ms, 871 µs and 655 ns            | 0.00%      |         |
| extension_attributes    | 164 ms, 92 µs and 340 ns             | 0.00%      |         |
| workspace_write_to_disk | 2 seconds, 124 ms, 663 µs and 674 ns | 40.00%     |         |
| workspace_toml          | 1 second, 665 ms, 167 µs and 506 ns  | 20.00%     |         |
| workspace_rustfmt       | 332 µs and 645 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
