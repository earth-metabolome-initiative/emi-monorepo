# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 33 seconds.
The slowest task was `Checking Generated Workspace` which took 26 seconds, 23 ms, 85 µs and 381 ns (78.34% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 438 ms, 666 µs and 420 ns | 10.35%     |         |
| SQL Workspace Generation     | 1 second, 888 ms, 902 µs and 3 ns    | 5.69%      |         |
| Formatting Workspace         | 1 second, 866 ms, 204 µs and 426 ns  | 5.62%      |         |
| Checking Generated Workspace | 26 seconds, 23 ms, 85 µs and 381 ns  | 78.34%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 284 ms, 693 µs and 143 ns (68.01% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 6 ms, 772 µs and 238 ns             | 0.36%      |         |
| model                         | 80 ms, 564 µs and 409 ns            | 4.27%      |         |
| relations_trait               | 12 ms, 809 µs and 545 ns            | 0.68%      |         |
| attributes                    | 16 ms, 769 µs and 589 ns            | 0.89%      |         |
| value_settable_trait          | 5 ms, 658 µs and 616 ns             | 0.30%      |         |
| insertable_key_settable_trait | 8 ms, 978 µs and 346 ns             | 0.48%      |         |
| buildable_key_settable_trait  | 48 ms, 899 µs and 794 ns            | 2.59%      |         |
| insertable                    | 27 ms, 954 µs and 285 ns            | 1.48%      |         |
| buildable                     | 104 ms, 136 µs and 210 ns           | 5.51%      |         |
| extension_attributes          | 11 ms, 35 µs and 844 ns             | 0.58%      |         |
| workspace_write_to_disk       | 1 second, 284 ms, 693 µs and 143 ns | 68.01%     |         |
| workspace_toml                | 280 ms, 389 µs and 961 ns           | 14.84%     |         |
| workspace_rustfmt             | 240 µs and 23 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
