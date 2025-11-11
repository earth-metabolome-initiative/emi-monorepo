# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 37 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 955 ms, 250 µs and 291 ns (67.57% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 359 ms, 373 µs and 627 ns  | 8.11%      |         |
| SQL Workspace Generation     | 6 seconds, 705 ms, 237 µs and 770 ns  | 16.22%     |         |
| Formatting Workspace         | 1 second, 571 ms, 676 µs and 825 ns   | 2.70%      |         |
| Checking Generated Workspace | 25 seconds, 955 ms, 250 µs and 291 ns | 67.57%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 777 ms, 460 µs and 534 ns (33.33% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 8 ms, 311 µs and 93 ns               | 0.00%      |         |
| model                         | 293 ms, 442 µs and 842 ns            | 0.00%      |         |
| relations_trait               | 17 ms, 685 µs and 627 ns             | 0.00%      |         |
| attributes                    | 294 ms, 904 µs and 414 ns            | 0.00%      |         |
| value_settable_trait          | 6 ms, 372 µs and 98 ns               | 0.00%      |         |
| insertable_key_settable_trait | 10 ms, 977 µs and 366 ns             | 0.00%      |         |
| insertable                    | 310 ms, 951 µs and 46 ns             | 0.00%      |         |
| buildable                     | 586 ms, 921 µs and 692 ns            | 0.00%      |         |
| extension_attributes          | 171 ms, 133 µs and 455 ns            | 0.00%      |         |
| workspace_write_to_disk       | 2 seconds, 777 ms, 460 µs and 534 ns | 33.33%     |         |
| workspace_toml                | 2 seconds, 226 ms, 732 µs and 380 ns | 33.33%     |         |
| workspace_rustfmt             | 345 µs and 223 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
