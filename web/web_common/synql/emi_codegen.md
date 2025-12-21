# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 20 seconds, 127 ms, 976 µs and 376 ns (75.75% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 974 ms, 489 µs and 134 ns  | 14.96%     |         |
| Schema Validation            | 1 second, 102 ms, 911 µs and 305 ns   | 4.15%      |         |
| SQL Workspace Generation     | 1 second, 178 ms, 734 µs and 779 ns   | 4.44%      |         |
| Formatting Workspace         | 187 ms, 493 µs and 15 ns              | 0.71%      |         |
| Checking Generated Workspace | 20 seconds, 127 ms, 976 µs and 376 ns | 75.75%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 158 ms, 197 µs and 51 ns (98.26% of all time).

| name               | time                               | percentage | comment |
|--------------------|------------------------------------|------------|---------|
| writing_crate_toml | 16 ms, 702 µs and 839 ns           | 1.42%      |         |
| writing_crate_lib  | 1 second, 158 ms, 197 µs and 51 ns | 98.26%     |         |
| workspace_toml     | 3 ms, 759 µs and 355 ns            | 0.32%      |         |
| workspace_rustfmt  | 75 µs and 534 ns                   | 0.01%      |         |

![Plot](emi_codegen.png)
