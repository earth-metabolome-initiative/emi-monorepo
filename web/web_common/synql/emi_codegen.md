# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 35 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 239 ms, 982 µs and 540 ns (76.33% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 2 seconds, 908 ms, 775 µs and 149 ns  | 8.15%      |         |
| SQL Workspace Generation     | 3 seconds, 427 ms, 9 µs and 965 ns    | 9.60%      |         |
| Formatting Workspace         | 2 seconds, 111 ms, 620 µs and 557 ns  | 5.92%      |         |
| Checking Generated Workspace | 27 seconds, 239 ms, 982 µs and 540 ns | 76.33%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 834 ms, 228 µs and 732 ns (82.70% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 6 ms, 470 µs and 752 ns              | 0.19%      |         |
| model                         | 79 ms, 87 µs and 531 ns              | 2.31%      |         |
| relations_trait               | 11 ms, 950 µs and 988 ns             | 0.35%      |         |
| attributes                    | 14 ms, 172 µs and 506 ns             | 0.41%      |         |
| value_settable_trait          | 5 ms, 125 µs and 471 ns              | 0.15%      |         |
| insertable_key_settable_trait | 8 ms, 276 µs and 999 ns              | 0.24%      |         |
| buildable_key_settable_trait  | 48 ms, 207 µs and 40 ns              | 1.41%      |         |
| insertable                    | 24 ms, 905 µs and 204 ns             | 0.73%      |         |
| buildable                     | 28 ms, 132 µs and 808 ns             | 0.82%      |         |
| extension_attributes          | 9 ms, 233 µs and 883 ns              | 0.27%      |         |
| workspace_write_to_disk       | 2 seconds, 834 ms, 228 µs and 732 ns | 82.70%     |         |
| workspace_toml                | 356 ms, 871 µs and 416 ns            | 10.41%     |         |
| workspace_rustfmt             | 346 µs and 635 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
