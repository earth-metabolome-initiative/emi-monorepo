# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 32 seconds.
The slowest task was `Checking Generated Workspace` which took 24 seconds, 719 ms, 374 µs and 360 ns (75.47% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 381 ms, 318 µs and 913 ns  | 10.32%     |         |
| SQL Workspace Generation     | 1 second, 967 ms, 413 µs and 17 ns    | 6.01%      |         |
| Formatting Workspace         | 2 seconds, 684 ms, 225 µs and 708 ns  | 8.20%      |         |
| Checking Generated Workspace | 24 seconds, 719 ms, 374 µs and 360 ns | 75.47%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 59 ms, 540 µs and 445 ns (53.85% of all time).

| name                          | time                               | percentage | comment |
|-------------------------------|------------------------------------|------------|---------|
| schema_macro                  | 16 ms, 997 µs and 278 ns           | 0.86%      |         |
| model                         | 88 ms, 88 µs and 370 ns            | 4.48%      |         |
| relations_trait               | 12 ms, 817 µs and 629 ns           | 0.65%      |         |
| attributes                    | 16 ms, 880 µs and 980 ns           | 0.86%      |         |
| value_settable_trait          | 5 ms, 690 µs and 891 ns            | 0.29%      |         |
| insertable_key_settable_trait | 8 ms, 956 µs and 280 ns            | 0.46%      |         |
| buildable_key_settable_trait  | 44 ms, 120 µs and 256 ns           | 2.24%      |         |
| insertable                    | 30 ms, 312 µs and 847 ns           | 1.54%      |         |
| buildable                     | 245 ms, 622 µs and 430 ns          | 12.48%     |         |
| extension_attributes          | 11 ms, 69 µs and 647 ns            | 0.56%      |         |
| workspace_write_to_disk       | 1 second, 59 ms, 540 µs and 445 ns | 53.85%     |         |
| workspace_toml                | 427 ms, 177 µs and 174 ns          | 21.71%     |         |
| workspace_rustfmt             | 138 µs and 790 ns                  | 0.01%      |         |

![Plot](emi_codegen.png)
