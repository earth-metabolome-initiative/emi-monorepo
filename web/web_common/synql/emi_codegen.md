# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 37 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 340 ms, 162 µs and 795 ns (76.48% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 420 ms, 709 µs and 490 ns  | 9.23%      |         |
| Schema Validation            | 922 ms, 904 µs and 689 ns             | 2.49%      |         |
| SQL Workspace Generation     | 1 second, 635 ms, 843 µs and 177 ns   | 4.41%      |         |
| Formatting Workspace         | 2 seconds, 734 ms, 350 µs and 982 ns  | 7.38%      |         |
| Checking Generated Workspace | 28 seconds, 340 ms, 162 µs and 795 ns | 76.48%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `model` which took 533 ms, 750 µs and 130 ns (32.63% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 22 ms, 787 µs and 788 ns  | 1.39%      |         |
| model                         | 533 ms, 750 µs and 130 ns | 32.63%     |         |
| transitive_extension_trait    | 5 ms, 111 µs and 291 ns   | 0.31%      |         |
| relations_trait               | 14 ms, 662 µs and 23 ns   | 0.90%      |         |
| attributes                    | 17 ms, 403 µs and 340 ns  | 1.06%      |         |
| value_settable_trait          | 5 ms, 223 µs and 20 ns    | 0.32%      |         |
| insertable_key_settable_trait | 8 ms, 615 µs and 425 ns   | 0.53%      |         |
| buildable_key_settable_trait  | 25 ms, 146 µs and 210 ns  | 1.54%      |         |
| insertable                    | 29 ms, 770 µs and 735 ns  | 1.82%      |         |
| buildable                     | 211 ms, 528 µs and 488 ns | 12.93%     |         |
| extension_attributes          | 11 ms, 568 µs and 366 ns  | 0.71%      |         |
| workspace_write_to_disk       | 502 ms, 10 µs and 233 ns  | 30.69%     |         |
| workspace_toml                | 248 ms, 154 µs and 359 ns | 15.17%     |         |
| workspace_rustfmt             | 111 µs and 769 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
