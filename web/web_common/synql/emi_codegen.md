# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 31 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 754 ms, 715 µs and 431 ns (81.16% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 383 ms, 742 µs and 335 ns  | 10.66%     |         |
| SQL Workspace Generation     | 727 ms, 585 µs and 130 ns             | 2.29%      |         |
| Formatting Workspace         | 1 second, 866 ms, 659 µs and 750 ns   | 5.88%      |         |
| Checking Generated Workspace | 25 seconds, 754 ms, 715 µs and 431 ns | 81.16%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 240 ms, 175 µs and 319 ns (33.01% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 6 ms, 744 µs and 37 ns    | 0.93%      |         |
| model                         | 81 ms, 88 µs and 469 ns   | 11.14%     |         |
| relations_trait               | 12 ms, 966 µs and 924 ns  | 1.78%      |         |
| attributes                    | 16 ms, 603 µs and 114 ns  | 2.28%      |         |
| value_settable_trait          | 5 ms, 536 µs and 763 ns   | 0.76%      |         |
| insertable_key_settable_trait | 8 ms, 835 µs and 316 ns   | 1.21%      |         |
| buildable_key_settable_trait  | 48 ms, 752 µs and 430 ns  | 6.70%      |         |
| insertable                    | 27 ms, 905 µs and 872 ns  | 3.84%      |         |
| buildable                     | 104 ms, 694 µs and 101 ns | 14.39%     |         |
| extension_attributes          | 10 ms, 990 µs and 679 ns  | 1.51%      |         |
| workspace_write_to_disk       | 163 ms, 197 µs and 753 ns | 22.43%     |         |
| workspace_toml                | 240 ms, 175 µs and 319 ns | 33.01%     |         |
| workspace_rustfmt             | 94 µs and 353 ns          | 0.01%      |         |

![Plot](emi_codegen.png)
