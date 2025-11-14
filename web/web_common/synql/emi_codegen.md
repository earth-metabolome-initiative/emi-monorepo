# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 36 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 22 ms, 513 µs and 335 ns (74.45% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 389 ms, 932 µs and 650 ns | 9.34%      |         |
| SQL Workspace Generation     | 3 seconds, 384 ms, 231 µs and 927 ns | 9.32%      |         |
| Formatting Workspace         | 2 seconds, 498 ms, 223 µs and 226 ns | 6.88%      |         |
| Checking Generated Workspace | 27 seconds, 22 ms, 513 µs and 335 ns | 74.45%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 649 ms, 445 µs and 868 ns (78.29% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 7 ms, 472 µs and 276 ns              | 0.22%      |         |
| model                         | 85 ms, 49 µs and 34 ns               | 2.51%      |         |
| relations_trait               | 13 ms, 516 µs and 37 ns              | 0.40%      |         |
| attributes                    | 17 ms, 427 µs and 330 ns             | 0.51%      |         |
| value_settable_trait          | 6 ms, 349 µs and 806 ns              | 0.19%      |         |
| insertable_key_settable_trait | 9 ms, 352 µs and 783 ns              | 0.28%      |         |
| buildable_key_settable_trait  | 50 ms, 75 µs and 955 ns              | 1.48%      |         |
| insertable                    | 34 ms, 20 µs and 858 ns              | 1.01%      |         |
| buildable                     | 220 ms, 467 µs and 129 ns            | 6.51%      |         |
| extension_attributes          | 11 ms, 475 µs and 905 ns             | 0.34%      |         |
| workspace_write_to_disk       | 2 seconds, 649 ms, 445 µs and 868 ns | 78.29%     |         |
| workspace_toml                | 279 ms, 468 µs and 278 ns            | 8.26%      |         |
| workspace_rustfmt             | 110 µs and 668 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
