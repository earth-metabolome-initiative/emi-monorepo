# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 34 seconds.
The slowest task was `Checking Generated Workspace` which took 26 seconds, 470 ms, 921 µs and 27 ns (77.00% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 462 ms, 405 µs and 426 ns | 10.07%     |         |
| SQL Workspace Generation     | 1 second, 700 ms, 793 µs and 527 ns  | 4.95%      |         |
| Formatting Workspace         | 2 seconds, 744 ms, 849 µs and 889 ns | 7.98%      |         |
| Checking Generated Workspace | 26 seconds, 470 ms, 921 µs and 27 ns | 77.00%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 991 ms, 688 µs and 209 ns (58.31% of all time).

| name                          | time                      | percentage | comment |
|-------------------------------|---------------------------|------------|---------|
| schema_macro                  | 6 ms, 757 µs and 760 ns   | 0.40%      |         |
| model                         | 84 ms, 683 µs and 303 ns  | 4.98%      |         |
| relations_trait               | 12 ms, 952 µs and 484 ns  | 0.76%      |         |
| attributes                    | 16 ms, 484 µs and 295 ns  | 0.97%      |         |
| value_settable_trait          | 5 ms, 650 µs and 178 ns   | 0.33%      |         |
| insertable_key_settable_trait | 8 ms, 841 µs and 427 ns   | 0.52%      |         |
| buildable_key_settable_trait  | 49 ms, 495 µs and 527 ns  | 2.91%      |         |
| insertable                    | 28 ms, 990 µs and 214 ns  | 1.70%      |         |
| buildable                     | 214 ms, 40 µs and 176 ns  | 12.58%     |         |
| extension_attributes          | 10 ms, 996 µs and 631 ns  | 0.65%      |         |
| workspace_write_to_disk       | 991 ms, 688 µs and 209 ns | 58.31%     |         |
| workspace_toml                | 270 ms, 95 µs and 334 ns  | 15.88%     |         |
| workspace_rustfmt             | 117 µs and 989 ns         | 0.01%      |         |

![Plot](emi_codegen.png)
