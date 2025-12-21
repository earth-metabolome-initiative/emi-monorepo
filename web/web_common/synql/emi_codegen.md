# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 19 seconds, 588 ms, 620 µs and 560 ns (74.61% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 985 ms, 726 µs and 127 ns  | 15.18%     |         |
| Schema Validation            | 1 second, 167 ms, 648 µs and 19 ns    | 4.45%      |         |
| SQL Workspace Generation     | 1 second, 328 ms, 297 µs and 323 ns   | 5.06%      |         |
| Formatting Workspace         | 185 ms, 243 µs and 972 ns             | 0.71%      |         |
| Checking Generated Workspace | 19 seconds, 588 ms, 620 µs and 560 ns | 74.61%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 305 ms, 889 µs and 469 ns (98.31% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 18 ms, 369 µs and 246 ns            | 1.38%      |         |
| writing_crate_lib  | 1 second, 305 ms, 889 µs and 469 ns | 98.31%     |         |
| workspace_toml     | 3 ms, 964 µs and 456 ns             | 0.30%      |         |
| workspace_rustfmt  | 74 µs and 152 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
