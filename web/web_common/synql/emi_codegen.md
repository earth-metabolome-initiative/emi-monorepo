# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 19 seconds, 711 ms, 665 µs and 642 ns (75.78% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 990 ms, 290 µs and 819 ns  | 15.34%     |         |
| Schema Validation            | 792 ms, 607 µs and 137 ns             | 3.05%      |         |
| SQL Workspace Generation     | 1 second, 320 ms, 480 µs and 604 ns   | 5.08%      |         |
| Formatting Workspace         | 195 ms, 384 µs and 436 ns             | 0.75%      |         |
| Checking Generated Workspace | 19 seconds, 711 ms, 665 µs and 642 ns | 75.78%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 300 ms, 243 µs and 54 ns (98.47% of all time).

| name               | time                               | percentage | comment |
|--------------------|------------------------------------|------------|---------|
| writing_crate_toml | 16 ms, 458 µs and 687 ns           | 1.25%      |         |
| writing_crate_lib  | 1 second, 300 ms, 243 µs and 54 ns | 98.47%     |         |
| workspace_toml     | 3 ms, 696 µs and 769 ns            | 0.28%      |         |
| workspace_rustfmt  | 82 µs and 94 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
