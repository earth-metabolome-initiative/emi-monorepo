# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 38 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 759 ms, 309 µs and 121 ns (73.93% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 480 ms, 730 µs and 582 ns  | 8.95%      |         |
| Schema Validation            | 965 ms, 265 µs and 56 ns              | 2.48%      |         |
| SQL Workspace Generation     | 2 seconds, 952 ms, 722 µs and 710 ns  | 7.59%      |         |
| Formatting Workspace         | 2 seconds, 740 ms, 509 µs and 693 ns  | 7.05%      |         |
| Checking Generated Workspace | 28 seconds, 759 ms, 309 µs and 121 ns | 73.93%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 748 ms, 961 µs and 210 ns (59.23% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 25 ms, 352 µs and 601 ns            | 0.86%      |         |
| model                         | 558 ms, 172 µs and 865 ns           | 18.90%     |         |
| transitive_extension_trait    | 5 ms, 30 µs and 848 ns              | 0.17%      |         |
| relations_trait               | 14 ms, 393 µs and 121 ns            | 0.49%      |         |
| attributes                    | 17 ms, 409 µs and 19 ns             | 0.59%      |         |
| value_settable_trait          | 5 ms, 202 µs and 513 ns             | 0.18%      |         |
| insertable_key_settable_trait | 8 ms, 626 µs and 235 ns             | 0.29%      |         |
| buildable_key_settable_trait  | 24 ms, 983 µs and 369 ns            | 0.85%      |         |
| insertable                    | 33 ms, 259 µs and 684 ns            | 1.13%      |         |
| buildable                     | 209 ms, 566 µs and 274 ns           | 7.10%      |         |
| extension_attributes          | 11 ms, 741 µs and 934 ns            | 0.40%      |         |
| workspace_write_to_disk       | 1 second, 748 ms, 961 µs and 210 ns | 59.23%     |         |
| workspace_toml                | 289 ms, 912 µs and 840 ns           | 9.82%      |         |
| workspace_rustfmt             | 110 µs and 197 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
