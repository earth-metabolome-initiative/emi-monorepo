# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 24 seconds.
The slowest task was `Checking Generated Workspace` which took 19 seconds, 219 ms, 647 µs and 869 ns (78.90% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 482 ms, 538 µs and 62 ns   | 14.30%     |         |
| Schema Validation            | 630 ms, 690 µs and 521 ns             | 2.59%      |         |
| SQL Workspace Generation     | 835 ms, 733 µs and 886 ns             | 3.43%      |         |
| Formatting Workspace         | 190 ms, 63 µs and 509 ns              | 0.78%      |         |
| Checking Generated Workspace | 19 seconds, 219 ms, 647 µs and 869 ns | 78.90%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 818 ms, 390 µs and 243 ns (97.92% of all time).

| name               | time                      | percentage | comment |
|--------------------|---------------------------|------------|---------|
| writing_crate_toml | 13 ms, 676 µs and 118 ns  | 1.64%      |         |
| writing_crate_lib  | 818 ms, 390 µs and 243 ns | 97.92%     |         |
| workspace_toml     | 3 ms, 598 µs and 190 ns   | 0.43%      |         |
| workspace_rustfmt  | 69 µs and 335 ns          | 0.01%      |         |

![Plot](emi_codegen.png)
