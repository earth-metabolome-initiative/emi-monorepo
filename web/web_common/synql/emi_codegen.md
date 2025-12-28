# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 20 seconds, 333 ms, 247 µs and 862 ns (76.02% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 900 ms, 87 µs and 502 ns   | 14.58%     |         |
| Schema Validation            | 760 ms, 436 µs and 769 ns             | 2.84%      |         |
| SQL Workspace Generation     | 1 second, 537 ms, 747 µs and 713 ns   | 5.75%      |         |
| Formatting Workspace         | 215 ms, 207 µs and 54 ns              | 0.80%      |         |
| Checking Generated Workspace | 20 seconds, 333 ms, 247 µs and 862 ns | 76.02%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 518 ms, 16 µs and 699 ns (98.72% of all time).

| name               | time                               | percentage | comment |
|--------------------|------------------------------------|------------|---------|
| writing_crate_toml | 16 ms, 640 µs and 981 ns           | 1.08%      |         |
| writing_crate_lib  | 1 second, 518 ms, 16 µs and 699 ns | 98.72%     |         |
| workspace_toml     | 3 ms, 32 µs and 185 ns             | 0.20%      |         |
| workspace_rustfmt  | 57 µs and 848 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
