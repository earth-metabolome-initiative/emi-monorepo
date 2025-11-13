# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 32 seconds.
The slowest task was `Checking Generated Workspace` which took 24 seconds, 417 ms, 382 µs and 818 ns (75.76% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 400 ms, 427 µs and 59 ns   | 10.55%     |         |
| SQL Workspace Generation     | 1 second, 640 ms, 124 µs and 236 ns   | 5.09%      |         |
| Formatting Workspace         | 2 seconds, 772 ms, 848 µs and 474 ns  | 8.60%      |         |
| Checking Generated Workspace | 24 seconds, 417 ms, 382 µs and 818 ns | 75.76%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 950 ms, 405 µs and 785 ns (57.95% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 6 ms, 773 µs and 969 ns   | 0.41%      |         |
| model                         | 86 ms, 994 µs and 347 ns  | 5.30%      |         |
| relations_trait               | 13 ms, 16 µs and 244 ns   | 0.79%      |         |
| attributes                    | 16 ms, 770 µs and 38 ns   | 1.02%      |         |
| value_settable_trait          | 5 ms, 544 µs and 896 ns   | 0.34%      |         |
| insertable_key_settable_trait | 8 ms, 844 µs and 137 ns   | 0.54%      |         |
| buildable_key_settable_trait  | 48 ms, 869 µs and 612 ns  | 2.98%      |         |
| insertable                    | 29 ms, 420 µs and 92 ns   | 1.79%      |         |
| buildable                     | 196 ms, 622 µs and 112 ns | 11.99%     |         |
| extension_attributes          | 11 ms, 132 µs and 572 ns  | 0.68%      |         |
| workspace_write_to_disk       | 950 ms, 405 µs and 785 ns | 57.95%     |         |
| workspace_toml                | 265 ms, 621 µs and 878 ns | 16.20%     |         |
| workspace_rustfmt             | 108 µs and 554 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
