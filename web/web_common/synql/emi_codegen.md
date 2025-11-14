# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 38 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 793 ms, 282 µs and 407 ns (72.38% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 423 ms, 202 µs and 513 ns  | 8.92%      |         |
| Schema Validation            | 946 ms, 1 µs and 691 ns               | 2.46%      |         |
| SQL Workspace Generation     | 3 seconds, 768 ms, 672 µs and 820 ns  | 9.81%      |         |
| Formatting Workspace         | 2 seconds, 466 ms, 171 µs and 234 ns  | 6.42%      |         |
| Checking Generated Workspace | 27 seconds, 793 ms, 282 µs and 407 ns | 72.38%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 437 ms, 814 µs and 406 ns (64.69% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 16 ms, 559 µs and 271 ns             | 0.44%      |         |
| model                         | 578 ms, 76 µs and 765 ns             | 15.34%     |         |
| relations_trait               | 15 ms, 796 µs and 378 ns             | 0.42%      |         |
| attributes                    | 17 ms, 577 µs and 900 ns             | 0.47%      |         |
| value_settable_trait          | 5 ms, 290 µs and 410 ns              | 0.14%      |         |
| insertable_key_settable_trait | 8 ms, 890 µs and 635 ns              | 0.24%      |         |
| buildable_key_settable_trait  | 31 ms, 717 µs and 480 ns             | 0.84%      |         |
| insertable                    | 29 ms, 894 µs and 767 ns             | 0.79%      |         |
| buildable                     | 247 ms, 548 µs and 949 ns            | 6.57%      |         |
| extension_attributes          | 11 ms, 865 µs and 140 ns             | 0.31%      |         |
| workspace_write_to_disk       | 2 seconds, 437 ms, 814 µs and 406 ns | 64.69%     |         |
| workspace_toml                | 367 ms, 544 µs and 73 ns             | 9.75%      |         |
| workspace_rustfmt             | 96 µs and 646 ns                     | 0.00%      |         |

![Plot](emi_codegen.png)
