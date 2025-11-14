# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 33 seconds.
The slowest task was `Checking Generated Workspace` which took 26 seconds, 232 ms, 112 µs and 629 ns (78.59% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 375 ms, 695 µs and 426 ns  | 10.11%     |         |
| SQL Workspace Generation     | 1 second, 234 ms, 189 µs and 752 ns   | 3.70%      |         |
| Formatting Workspace         | 2 seconds, 534 ms, 488 µs and 944 ns  | 7.59%      |         |
| Checking Generated Workspace | 26 seconds, 232 ms, 112 µs and 629 ns | 78.59%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 528 ms, 765 µs and 654 ns (42.84% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 10 ms, 71 µs and 14 ns    | 0.82%      |         |
| model                         | 81 ms, 400 µs and 108 ns  | 6.60%      |         |
| relations_trait               | 12 ms, 952 µs and 74 ns   | 1.05%      |         |
| attributes                    | 16 ms, 933 µs and 768 ns  | 1.37%      |         |
| value_settable_trait          | 5 ms, 529 µs and 454 ns   | 0.45%      |         |
| insertable_key_settable_trait | 8 ms, 808 µs and 796 ns   | 0.71%      |         |
| buildable_key_settable_trait  | 49 ms, 57 µs and 52 ns    | 3.97%      |         |
| insertable                    | 28 ms, 242 µs and 241 ns  | 2.29%      |         |
| buildable                     | 207 ms, 460 µs and 336 ns | 16.81%     |         |
| extension_attributes          | 10 ms, 877 µs and 605 ns  | 0.88%      |         |
| workspace_write_to_disk       | 528 ms, 765 µs and 654 ns | 42.84%     |         |
| workspace_toml                | 273 ms, 990 µs and 576 ns | 22.20%     |         |
| workspace_rustfmt             | 101 µs and 74 ns          | 0.01%      |         |

![Plot](emi_codegen.png)
