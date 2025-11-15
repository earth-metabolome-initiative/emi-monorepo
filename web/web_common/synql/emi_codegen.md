# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 39 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 301 ms, 1 µs and 549 ns (72.32% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 427 ms, 688 µs and 333 ns | 8.76%      |         |
| Schema Validation            | 993 ms, 8 µs and 65 ns               | 2.54%      |         |
| SQL Workspace Generation     | 3 seconds, 481 ms, 820 µs and 624 ns | 8.90%      |         |
| Formatting Workspace         | 2 seconds, 929 ms, 134 µs and 592 ns | 7.49%      |         |
| Checking Generated Workspace | 28 seconds, 301 ms, 1 µs and 549 ns  | 72.32%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 115 ms, 284 µs and 621 ns (60.75% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 17 ms, 34 µs and 300 ns              | 0.49%      |         |
| model                         | 592 ms, 713 µs and 690 ns            | 17.02%     |         |
| transitive_extension_trait    | 5 ms, 282 µs and 129 ns              | 0.15%      |         |
| relations_trait               | 15 ms, 925 µs and 187 ns             | 0.46%      |         |
| attributes                    | 18 ms, 43 µs and 940 ns              | 0.52%      |         |
| value_settable_trait          | 5 ms, 460 µs and 276 ns              | 0.16%      |         |
| insertable_key_settable_trait | 9 ms, 27 µs and 775 ns               | 0.26%      |         |
| buildable_key_settable_trait  | 29 ms, 525 µs and 166 ns             | 0.85%      |         |
| insertable                    | 30 ms, 752 µs and 311 ns             | 0.88%      |         |
| buildable                     | 252 ms, 692 µs and 586 ns            | 7.26%      |         |
| extension_attributes          | 12 ms, 110 µs and 513 ns             | 0.35%      |         |
| workspace_write_to_disk       | 2 seconds, 115 ms, 284 µs and 621 ns | 60.75%     |         |
| workspace_toml                | 377 ms, 863 µs and 852 ns            | 10.85%     |         |
| workspace_rustfmt             | 104 µs and 278 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
