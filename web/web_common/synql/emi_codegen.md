# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 29 seconds.
The slowest task was `Checking Generated Workspace` which took 22 seconds, 395 ms, 8 µs and 336 ns (77.09% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 916 ms, 545 µs and 790 ns | 13.48%     |         |
| Schema Validation            | 1 second, 91 ms, 232 µs and 363 ns   | 3.76%      |         |
| SQL Workspace Generation     | 1 second, 449 ms, 807 µs and 844 ns  | 4.99%      |         |
| Formatting Workspace         | 196 ms, 901 µs and 454 ns            | 0.68%      |         |
| Checking Generated Workspace | 22 seconds, 395 ms, 8 µs and 336 ns  | 77.09%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 425 ms, 649 µs and 835 ns (98.33% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 19 ms, 248 µs and 898 ns            | 1.33%      |         |
| writing_crate_lib  | 1 second, 425 ms, 649 µs and 835 ns | 98.33%     |         |
| workspace_toml     | 4 ms, 807 µs and 517 ns             | 0.33%      |         |
| workspace_rustfmt  | 101 µs and 594 ns                   | 0.01%      |         |

![Plot](emi_codegen.png)
