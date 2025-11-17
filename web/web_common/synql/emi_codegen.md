# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 30 seconds.
The slowest task was `Checking Generated Workspace` which took 20 seconds, 657 ms, 258 µs and 233 ns (67.17% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 456 ms, 579 µs and 116 ns  | 11.24%     |         |
| Schema Validation            | 951 ms, 376 µs and 781 ns             | 3.09%      |         |
| SQL Workspace Generation     | 2 seconds, 872 ms, 346 µs and 640 ns  | 9.34%      |         |
| Formatting Workspace         | 2 seconds, 817 ms, 206 µs and 156 ns  | 9.16%      |         |
| Checking Generated Workspace | 20 seconds, 657 ms, 258 µs and 233 ns | 67.17%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 671 ms, 977 µs and 621 ns (58.21% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 25 ms, 729 µs and 206 ns            | 0.90%      |         |
| model                         | 552 ms, 727 µs and 63 ns            | 19.24%     |         |
| transitive_extension_trait    | 5 ms, 48 µs and 548 ns              | 0.18%      |         |
| relations_trait               | 14 ms, 270 µs and 708 ns            | 0.50%      |         |
| attributes                    | 17 ms, 286 µs and 341 ns            | 0.60%      |         |
| value_settable_trait          | 5 ms, 721 µs and 307 ns             | 0.20%      |         |
| insertable_key_settable_trait | 8 ms, 730 µs and 46 ns              | 0.30%      |         |
| buildable_key_settable_trait  | 24 ms, 853 µs and 629 ns            | 0.87%      |         |
| insertable                    | 30 ms, 875 µs and 832 ns            | 1.07%      |         |
| buildable                     | 209 ms, 793 µs and 233 ns           | 7.30%      |         |
| extension_attributes          | 11 ms, 577 µs and 578 ns            | 0.40%      |         |
| workspace_write_to_disk       | 1 second, 671 ms, 977 µs and 621 ns | 58.21%     |         |
| workspace_toml                | 293 ms, 650 µs and 388 ns           | 10.22%     |         |
| workspace_rustfmt             | 105 µs and 140 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
