# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was a minute.
The slowest task was `Checking Generated Workspace` which took 53 seconds, 415 ms, 561 µs and 906 ns (85.18% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 679 ms, 710 µs and 818 ns  | 5.87%      |         |
| Schema Validation            | 930 ms, 579 µs and 145 ns             | 1.48%      |         |
| SQL Workspace Generation     | 1 second, 836 ms, 130 µs and 267 ns   | 2.93%      |         |
| Formatting Workspace         | 2 seconds, 846 ms, 479 µs and 854 ns  | 4.54%      |         |
| Checking Generated Workspace | 53 seconds, 415 ms, 561 µs and 906 ns | 85.18%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 625 ms, 70 µs and 410 ns (34.04% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 25 ms, 602 µs and 21 ns   | 1.39%      |         |
| model                         | 557 ms, 52 µs and 901 ns  | 30.34%     |         |
| transitive_extension_trait    | 5 ms, 363 µs and 482 ns   | 0.29%      |         |
| relations_trait               | 14 ms, 334 µs and 732 ns  | 0.78%      |         |
| attributes                    | 17 ms, 822 µs and 186 ns  | 0.97%      |         |
| value_settable_trait          | 5 ms, 705 µs and 417 ns   | 0.31%      |         |
| insertable_key_settable_trait | 8 ms, 703 µs and 408 ns   | 0.47%      |         |
| buildable_key_settable_trait  | 24 ms, 864 µs and 246 ns  | 1.35%      |         |
| insertable                    | 30 ms, 988 µs and 821 ns  | 1.69%      |         |
| buildable                     | 209 ms, 122 µs and 312 ns | 11.39%     |         |
| extension_attributes          | 11 ms, 933 µs and 800 ns  | 0.65%      |         |
| workspace_write_to_disk       | 625 ms, 70 µs and 410 ns  | 34.04%     |         |
| workspace_toml                | 299 ms, 451 µs and 86 ns  | 16.31%     |         |
| workspace_rustfmt             | 115 µs and 445 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
