# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 31 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 53 ms, 338 µs and 479 ns (79.81% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 879 ms, 594 µs and 12 ns  | 12.36%     |         |
| Schema Validation            | 1 second, 15 ms, 173 µs and 474 ns   | 3.23%      |         |
| SQL Workspace Generation     | 1 second, 243 ms, 473 µs and 44 ns   | 3.96%      |         |
| Formatting Workspace         | 200 ms, 190 µs and 966 ns            | 0.64%      |         |
| Checking Generated Workspace | 25 seconds, 53 ms, 338 µs and 479 ns | 79.81%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 220 ms, 719 µs and 234 ns (98.17% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 17 ms, 506 µs and 270 ns            | 1.41%      |         |
| writing_crate_lib  | 1 second, 220 ms, 719 µs and 234 ns | 98.17%     |         |
| workspace_toml     | 5 ms, 137 µs and 724 ns             | 0.41%      |         |
| workspace_rustfmt  | 109 µs and 816 ns                   | 0.01%      |         |

![Plot](emi_codegen.png)
