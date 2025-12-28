# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 18 seconds, 443 ms, 921 µs and 431 ns (77.35% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 315 ms, 411 µs and 124 ns  | 13.90%     |         |
| Schema Validation            | 682 ms, 856 µs and 747 ns             | 2.86%      |         |
| SQL Workspace Generation     | 1 second, 196 ms, 293 µs and 568 ns   | 5.02%      |         |
| Formatting Workspace         | 206 ms, 357 µs and 345 ns             | 0.87%      |         |
| Checking Generated Workspace | 18 seconds, 443 ms, 921 µs and 431 ns | 77.35%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 177 ms, 1 µs and 954 ns (98.39% of all time).

| name               | time                              | percentage | comment |
|--------------------|-----------------------------------|------------|---------|
| writing_crate_toml | 15 ms, 708 µs and 159 ns          | 1.31%      |         |
| writing_crate_lib  | 1 second, 177 ms, 1 µs and 954 ns | 98.39%     |         |
| workspace_toml     | 3 ms, 517 µs and 195 ns           | 0.29%      |         |
| workspace_rustfmt  | 66 µs and 260 ns                  | 0.01%      |         |

![Plot](emi_codegen.png)
