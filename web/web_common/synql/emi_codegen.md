# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 41 seconds.
The slowest task was `Checking Generated Workspace` which took 30 seconds, 505 ms, 504 µs and 961 ns (72.98% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 451 ms, 188 µs and 893 ns  | 8.26%      |         |
| Schema Validation            | 959 ms, 384 µs and 494 ns             | 2.30%      |         |
| SQL Workspace Generation     | 4 seconds, 41 ms, 825 µs and 295 ns   | 9.67%      |         |
| Formatting Workspace         | 2 seconds, 843 ms, 599 µs and 612 ns  | 6.80%      |         |
| Checking Generated Workspace | 30 seconds, 505 ms, 504 µs and 961 ns | 72.98%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 769 ms, 282 µs and 68 ns (68.52% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 24 ms, 972 µs and 517 ns            | 0.62%      |         |
| model                         | 573 ms, 13 µs and 533 ns            | 14.18%     |         |
| transitive_extension_trait    | 5 ms, 122 µs and 876 ns             | 0.13%      |         |
| relations_trait               | 14 ms, 349 µs and 874 ns            | 0.36%      |         |
| attributes                    | 17 ms, 335 µs and 453 ns            | 0.43%      |         |
| value_settable_trait          | 5 ms, 157 µs and 579 ns             | 0.13%      |         |
| insertable_key_settable_trait | 8 ms, 575 µs and 405 ns             | 0.21%      |         |
| buildable_key_settable_trait  | 24 ms, 833 µs and 433 ns            | 0.61%      |         |
| insertable                    | 37 ms, 98 µs and 410 ns             | 0.92%      |         |
| buildable                     | 227 ms, 322 µs and 996 ns           | 5.62%      |         |
| extension_attributes          | 11 ms, 737 µs and 506 ns            | 0.29%      |         |
| workspace_write_to_disk       | 2 seconds, 769 ms, 282 µs and 68 ns | 68.52%     |         |
| workspace_toml                | 322 ms, 922 µs and 322 ns           | 7.99%      |         |
| workspace_rustfmt             | 101 µs and 323 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
