# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 677 ms, 551 µs and 35 ns (76.17% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 453 ms, 918 µs and 281 ns | 14.88%     |         |
| Schema Validation            | 680 ms, 113 µs and 15 ns             | 2.93%      |         |
| SQL Workspace Generation     | 1 second, 194 ms, 306 µs and 830 ns  | 5.15%      |         |
| Formatting Workspace         | 203 ms, 529 µs and 673 ns            | 0.88%      |         |
| Checking Generated Workspace | 17 seconds, 677 ms, 551 µs and 35 ns | 76.17%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 174 ms, 722 µs and 575 ns (98.36% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 15 ms, 920 µs and 767 ns            | 1.33%      |         |
| writing_crate_lib  | 1 second, 174 ms, 722 µs and 575 ns | 98.36%     |         |
| workspace_toml     | 3 ms, 597 µs and 258 ns             | 0.30%      |         |
| workspace_rustfmt  | 66 µs and 230 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
