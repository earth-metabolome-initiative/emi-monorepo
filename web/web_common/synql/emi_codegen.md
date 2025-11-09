# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 38 seconds.
The slowest task was `Checking Generated Workspace` which took 24 seconds, 758 ms, 266 µs and 458 ns (63.16% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 429 ms, 230 µs and 720 ns  | 7.89%      |         |
| SQL Workspace Generation                    | 7 seconds, 836 ms, 441 µs and 824 ns  | 18.42%     |         |
| Formatting and Checking Generated Workspace | 1 second, 979 ms, 247 µs and 726 ns   | 2.63%      |         |
| Checking Generated Workspace                | 24 seconds, 758 ms, 266 µs and 458 ns | 63.16%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 4 seconds, 585 ms, 443 µs and 135 ns (57.14% of all time).

| name                    | time                                 | percentage | comment |
|-------------------------|--------------------------------------|------------|---------|
| schema_macro            | 34 ms, 695 µs and 786 ns             | 0.00%      |         |
| model                   | 126 ms, 754 µs and 203 ns            | 0.00%      |         |
| relations_trait         | 46 ms, 26 µs and 646 ns              | 0.00%      |         |
| attributes              | 52 ms, 97 µs and 661 ns              | 0.00%      |         |
| value_settable_trait    | 16 ms, 559 µs and 210 ns             | 0.00%      |         |
| insertable              | 38 ms, 315 µs and 727 ns             | 0.00%      |         |
| buildable               | 95 ms, 179 µs and 394 ns             | 0.00%      |         |
| extension_attributes    | 35 ms, 369 µs and 150 ns             | 0.00%      |         |
| workspace_write_to_disk | 2 seconds, 805 ms, 698 µs and 22 ns  | 28.57%     |         |
| workspace_toml          | 4 seconds, 585 ms, 443 µs and 135 ns | 57.14%     |         |
| workspace_rustfmt       | 302 µs and 890 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
