# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 42 seconds.
The slowest task was `Checking Generated Workspace` which took 36 seconds, 192 ms, 361 µs and 149 ns (85.96% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 543 ms, 999 µs and 911 ns  | 8.42%      |         |
| Schema Validation            | 961 ms, 789 µs and 945 ns             | 2.28%      |         |
| SQL Workspace Generation     | 1 second, 214 ms, 606 µs and 600 ns   | 2.88%      |         |
| Formatting Workspace         | 190 ms, 682 µs and 51 ns              | 0.45%      |         |
| Checking Generated Workspace | 36 seconds, 192 ms, 361 µs and 149 ns | 85.96%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 192 ms, 753 µs and 734 ns (98.20% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 17 ms, 631 µs and 369 ns            | 1.45%      |         |
| writing_crate_lib  | 1 second, 192 ms, 753 µs and 734 ns | 98.20%     |         |
| workspace_toml     | 4 ms, 143 µs and 408 ns             | 0.34%      |         |
| workspace_rustfmt  | 78 µs and 89 ns                     | 0.01%      |         |

![Plot](emi_codegen.png)
