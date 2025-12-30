# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 22 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 255 ms, 955 µs and 106 ns (75.57% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 405 ms, 501 µs and 838 ns  | 14.91%     |         |
| Schema Validation            | 702 ms, 872 µs and 464 ns             | 3.08%      |         |
| SQL Workspace Generation     | 1 second, 260 ms, 930 µs and 389 ns   | 5.52%      |         |
| Formatting Workspace         | 208 ms, 185 µs and 407 ns             | 0.91%      |         |
| Checking Generated Workspace | 17 seconds, 255 ms, 955 µs and 106 ns | 75.57%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 231 ms, 6 µs and 46 ns (97.63% of all time).

| name                    | time                             | percentage | comment |
|-------------------------|----------------------------------|------------|---------|
| writing_crate_toml      | 15 ms, 98 µs and 630 ns          | 1.20%      |         |
| writing_crate_lib       | 1 second, 231 ms, 6 µs and 46 ns | 97.63%     |         |
| writing_sink_crate_toml | 728 µs and 483 ns                | 0.06%      |         |
| writing_sink_crate_lib  | 11 ms, 150 µs and 249 ns         | 0.88%      |         |
| workspace_toml          | 2 ms, 893 µs and 761 ns          | 0.23%      |         |
| workspace_rustfmt       | 53 µs and 220 ns                 | 0.00%      |         |

![Plot](emi_codegen.png)
