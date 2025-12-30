# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 18 seconds, 448 ms, 801 µs and 872 ns (77.10% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 346 ms, 828 µs and 398 ns  | 13.99%     |         |
| Schema Validation            | 689 ms, 441 µs and 841 ns             | 2.88%      |         |
| SQL Workspace Generation     | 1 second, 238 ms, 93 µs and 888 ns    | 5.17%      |         |
| Formatting Workspace         | 203 ms, 884 µs and 389 ns             | 0.85%      |         |
| Checking Generated Workspace | 18 seconds, 448 ms, 801 µs and 872 ns | 77.10%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 208 ms, 365 µs and 138 ns (97.60% of all time).

| name                    | time                                | percentage | comment |
|-------------------------|-------------------------------------|------------|---------|
| writing_crate_toml      | 14 ms, 885 µs and 604 ns            | 1.20%      |         |
| writing_crate_lib       | 1 second, 208 ms, 365 µs and 138 ns | 97.60%     |         |
| writing_sink_crate_toml | 730 µs and 716 ns                   | 0.06%      |         |
| writing_sink_crate_lib  | 11 ms, 165 µs and 530 ns            | 0.90%      |         |
| workspace_toml          | 2 ms, 892 µs and 548 ns             | 0.23%      |         |
| workspace_rustfmt       | 54 µs and 352 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
