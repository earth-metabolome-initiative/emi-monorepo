# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 34 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 502 ms, 518 µs and 980 ns (80.29% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 396 ms, 98 µs and 994 ns   | 9.91%      |         |
| SQL Workspace Generation     | 859 ms, 72 µs and 855 ns              | 2.51%      |         |
| Formatting Workspace         | 2 seconds, 497 ms, 133 µs and 708 ns  | 7.29%      |         |
| Checking Generated Workspace | 27 seconds, 502 ms, 518 µs and 980 ns | 80.29%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 272 ms, 397 µs and 713 ns (31.71% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 7 ms, 15 µs and 930 ns    | 0.82%      |         |
| model                         | 87 ms, 85 µs and 60 ns    | 10.14%     |         |
| relations_trait               | 13 ms, 20 µs and 60 ns    | 1.52%      |         |
| attributes                    | 16 ms, 981 µs and 604 ns  | 1.98%      |         |
| value_settable_trait          | 5 ms, 823 µs and 279 ns   | 0.68%      |         |
| insertable_key_settable_trait | 8 ms, 996 µs and 503 ns   | 1.05%      |         |
| buildable_key_settable_trait  | 49 ms, 395 µs and 787 ns  | 5.75%      |         |
| insertable                    | 28 ms, 688 µs and 245 ns  | 3.34%      |         |
| buildable                     | 211 ms, 378 µs and 229 ns | 24.61%     |         |
| extension_attributes          | 11 ms, 142 µs and 768 ns  | 1.30%      |         |
| workspace_write_to_disk       | 147 ms, 10 µs and 449 ns  | 17.11%     |         |
| workspace_toml                | 272 ms, 397 µs and 713 ns | 31.71%     |         |
| workspace_rustfmt             | 137 µs and 228 ns         | 0.02%      |         |

![Plot](emi_codegen.png)
