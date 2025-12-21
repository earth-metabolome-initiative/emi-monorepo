# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 20 seconds, 429 ms, 528 µs and 546 ns (75.86% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 993 ms, 985 µs and 369 ns  | 14.83%     |         |
| Schema Validation            | 1 second, 105 ms, 219 µs and 321 ns   | 4.10%      |         |
| SQL Workspace Generation     | 1 second, 213 ms, 192 µs and 208 ns   | 4.50%      |         |
| Formatting Workspace         | 189 ms, 836 µs and 225 ns             | 0.70%      |         |
| Checking Generated Workspace | 20 seconds, 429 ms, 528 µs and 546 ns | 75.86%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 192 ms, 142 µs and 279 ns (98.26% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 17 ms, 24 µs and 544 ns             | 1.40%      |         |
| writing_crate_lib  | 1 second, 192 ms, 142 µs and 279 ns | 98.26%     |         |
| workspace_toml     | 3 ms, 952 µs and 485 ns             | 0.33%      |         |
| workspace_rustfmt  | 72 µs and 900 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
