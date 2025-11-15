# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 40 seconds.
The slowest task was `Checking Generated Workspace` which took 29 seconds, 572 ms, 78 µs and 808 ns (72.78% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 347 ms, 756 µs and 821 ns | 8.24%      |         |
| Schema Validation            | 940 ms, 603 µs and 849 ns            | 2.32%      |         |
| SQL Workspace Generation     | 3 seconds, 949 ms, 567 µs and 176 ns | 9.72%      |         |
| Formatting Workspace         | 2 seconds, 820 ms, 211 µs and 821 ns | 6.94%      |         |
| Checking Generated Workspace | 29 seconds, 572 ms, 78 µs and 808 ns | 72.78%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 618 ms, 719 µs and 189 ns (66.30% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 16 ms, 919 µs and 629 ns             | 0.43%      |         |
| model                         | 574 ms, 252 µs and 996 ns            | 14.54%     |         |
| transitive_extension_trait    | 5 ms, 291 µs and 79 ns               | 0.13%      |         |
| relations_trait               | 15 ms, 938 µs and 894 ns             | 0.40%      |         |
| attributes                    | 18 ms, 53 µs and 376 ns              | 0.46%      |         |
| value_settable_trait          | 5 ms, 691 µs and 998 ns              | 0.14%      |         |
| insertable_key_settable_trait | 9 ms, 167 µs and 580 ns              | 0.23%      |         |
| buildable_key_settable_trait  | 28 ms, 895 µs and 276 ns             | 0.73%      |         |
| insertable                    | 30 ms, 622 µs and 535 ns             | 0.78%      |         |
| buildable                     | 247 ms, 356 µs and 564 ns            | 6.26%      |         |
| extension_attributes          | 12 ms, 232 µs and 741 ns             | 0.31%      |         |
| workspace_write_to_disk       | 2 seconds, 618 ms, 719 µs and 189 ns | 66.30%     |         |
| workspace_toml                | 366 ms, 328 µs and 192 ns            | 9.28%      |         |
| workspace_rustfmt             | 97 µs and 127 ns                     | 0.00%      |         |

![Plot](emi_codegen.png)
