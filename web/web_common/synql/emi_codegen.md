# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 517 ms, 824 µs and 894 ns (76.12% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 335 ms, 961 µs and 21 ns   | 14.50%     |         |
| Schema Validation            | 664 ms, 879 µs and 755 ns             | 2.89%      |         |
| SQL Workspace Generation     | 1 second, 263 ms, 750 µs and 271 ns   | 5.49%      |         |
| Formatting Workspace         | 231 ms, 239 µs and 70 ns              | 1.00%      |         |
| Checking Generated Workspace | 17 seconds, 517 ms, 824 µs and 894 ns | 76.12%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 243 ms, 632 µs and 218 ns (98.41% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 16 ms, 485 µs and 795 ns            | 1.30%      |         |
| writing_crate_lib  | 1 second, 243 ms, 632 µs and 218 ns | 98.41%     |         |
| workspace_toml     | 3 ms, 555 µs and 111 ns             | 0.28%      |         |
| workspace_rustfmt  | 77 µs and 147 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
