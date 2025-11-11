# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 35 seconds.
The slowest task was `Checking Generated Workspace` which took 25 seconds, 510 ms, 674 µs and 764 ns (71.43% of all time).

| name                                        | time                                  | percentage | comment |
|---------------------------------------------|---------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 432 ms, 467 µs and 184 ns  | 8.57%      |         |
| SQL Workspace Generation                    | 4 seconds, 463 ms, 387 µs and 691 ns  | 11.43%     |         |
| Formatting and Checking Generated Workspace | 1 second, 659 ms, 908 µs and 891 ns   | 2.86%      |         |
| Checking Generated Workspace                | 25 seconds, 510 ms, 674 µs and 764 ns | 71.43%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_toml` which took 2 seconds, 272 ms, 592 µs and 410 ns (50.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 8 ms, 393 µs and 309 ns              | 0.00%      |         |
| model                         | 311 ms, 635 µs and 564 ns            | 0.00%      |         |
| relations_trait               | 18 ms, 40 µs and 909 ns              | 0.00%      |         |
| attributes                    | 302 ms, 279 µs and 33 ns             | 0.00%      |         |
| value_settable_trait          | 6 ms, 66 µs and 607 ns               | 0.00%      |         |
| insertable_key_settable_trait | 10 ms, 285 µs and 770 ns             | 0.00%      |         |
| insertable                    | 319 ms, 932 µs and 781 ns            | 0.00%      |         |
| buildable                     | 604 ms, 323 µs and 307 ns            | 0.00%      |         |
| extension_attributes          | 174 ms, 661 µs and 436 ns            | 0.00%      |         |
| workspace_write_to_disk       | 434 ms, 835 µs and 117 ns            | 0.00%      |         |
| workspace_toml                | 2 seconds, 272 ms, 592 µs and 410 ns | 50.00%     |         |
| workspace_rustfmt             | 341 µs and 448 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
