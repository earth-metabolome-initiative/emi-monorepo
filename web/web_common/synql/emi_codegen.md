# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 40 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 719 ms, 20 µs and 212 ns (71.37% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 405 ms, 866 µs and 16 ns  | 8.46%      |         |
| Schema Validation            | 954 ms, 936 µs and 647 ns            | 2.37%      |         |
| SQL Workspace Generation     | 4 seconds, 688 ms, 149 µs and 831 ns | 11.65%     |         |
| Formatting Workspace         | 2 seconds, 473 ms, 367 µs and 864 ns | 6.15%      |         |
| Checking Generated Workspace | 28 seconds, 719 ms, 20 µs and 212 ns | 71.37%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 3 seconds, 402 ms, 979 µs and 725 ns (72.59% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 16 ms, 438 µs and 315 ns             | 0.35%      |         |
| model                         | 546 ms, 659 µs and 48 ns             | 11.66%     |         |
| transitive_extension_trait    | 5 ms, 141 µs and 382 ns              | 0.11%      |         |
| relations_trait               | 15 ms, 172 µs and 642 ns             | 0.32%      |         |
| attributes                    | 17 ms, 559 µs and 126 ns             | 0.37%      |         |
| value_settable_trait          | 5 ms, 247 µs and 570 ns              | 0.11%      |         |
| insertable_key_settable_trait | 8 ms, 623 µs and 171 ns              | 0.18%      |         |
| buildable_key_settable_trait  | 28 ms, 568 µs and 95 ns              | 0.61%      |         |
| insertable                    | 29 ms, 572 µs and 490 ns             | 0.63%      |         |
| buildable                     | 244 ms, 247 µs and 51 ns             | 5.21%      |         |
| extension_attributes          | 11 ms, 900 µs and 961 ns             | 0.25%      |         |
| workspace_write_to_disk       | 3 seconds, 402 ms, 979 µs and 725 ns | 72.59%     |         |
| workspace_toml                | 355 ms, 938 µs and 812 ns            | 7.59%      |         |
| workspace_rustfmt             | 101 µs and 443 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
