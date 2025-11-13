# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 35 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 323 ms, 970 µs and 261 ns (79.84% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 350 ms, 871 µs and 440 ns  | 9.45%      |         |
| SQL Workspace Generation     | 1 second, 31 ms, 87 µs and 975 ns     | 2.91%      |         |
| Formatting Workspace         | 2 seconds, 768 ms, 394 µs and 57 ns   | 7.80%      |         |
| Checking Generated Workspace | 28 seconds, 323 ms, 970 µs and 261 ns | 79.84%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 312 ms, 670 µs and 945 ns (30.32% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 6 ms, 743 µs and 353 ns   | 0.65%      |         |
| model                         | 83 ms, 872 µs and 350 ns  | 8.13%      |         |
| relations_trait               | 12 ms, 832 µs and 315 ns  | 1.24%      |         |
| attributes                    | 16 ms, 491 µs and 859 ns  | 1.60%      |         |
| value_settable_trait          | 5 ms, 565 µs and 966 ns   | 0.54%      |         |
| insertable_key_settable_trait | 8 ms, 776 µs and 859 ns   | 0.85%      |         |
| buildable_key_settable_trait  | 49 ms, 74 µs and 700 ns   | 4.76%      |         |
| insertable                    | 34 ms, 153 µs and 697 ns  | 3.31%      |         |
| buildable                     | 222 ms, 736 µs and 332 ns | 21.60%     |         |
| extension_attributes          | 11 ms, 176 µs and 103 ns  | 1.08%      |         |
| workspace_write_to_disk       | 312 ms, 670 µs and 945 ns | 30.32%     |         |
| workspace_toml                | 266 ms, 891 µs and 612 ns | 25.88%     |         |
| workspace_rustfmt             | 101 µs and 884 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
