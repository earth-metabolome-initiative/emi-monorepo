# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 689 ms, 775 µs and 597 ns (67.56% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 523 ms, 906 µs and 284 ns  | 13.46%     |         |
| Schema Validation            | 937 ms, 11 µs and 841 ns              | 3.58%      |         |
| SQL Workspace Generation     | 1 second, 516 ms, 50 µs and 3 ns      | 5.79%      |         |
| Formatting Workspace         | 2 seconds, 517 ms, 723 µs and 829 ns  | 9.62%      |         |
| Checking Generated Workspace | 17 seconds, 689 ms, 775 µs and 597 ns | 67.56%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `model` which took 567 ms, 8 µs and 413 ns (37.40% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 16 ms, 824 µs and 401 ns  | 1.11%      |         |
| model                         | 567 ms, 8 µs and 413 ns   | 37.40%     |         |
| relations_trait               | 12 ms, 927 µs and 726 ns  | 0.85%      |         |
| attributes                    | 21 ms, 399 µs and 245 ns  | 1.41%      |         |
| value_settable_trait          | 5 ms, 186 µs and 706 ns   | 0.34%      |         |
| insertable_key_settable_trait | 8 ms, 924 µs and 849 ns   | 0.59%      |         |
| buildable_key_settable_trait  | 31 ms, 884 µs and 287 ns  | 2.10%      |         |
| insertable                    | 30 ms, 610 µs and 372 ns  | 2.02%      |         |
| buildable                     | 248 ms, 434 µs and 970 ns | 16.39%     |         |
| extension_attributes          | 11 ms, 964 µs and 833 ns  | 0.79%      |         |
| workspace_write_to_disk       | 186 ms, 879 µs and 132 ns | 12.33%     |         |
| workspace_toml                | 373 ms, 902 µs and 954 ns | 24.66%     |         |
| workspace_rustfmt             | 102 µs and 115 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
