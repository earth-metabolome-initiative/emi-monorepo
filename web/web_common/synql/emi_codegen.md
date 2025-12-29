# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was a minute.
The slowest task was `Checking Generated Workspace` which took 1 minute, 1 second, 342 ms, 254 µs and 219 ns (91.91% of all time).

| name                         | time                                          | percentage | comment |
|------------------------------|-----------------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 327 ms, 494 µs and 317 ns          | 4.99%      |         |
| Schema Validation            | 677 ms, 589 µs and 610 ns                     | 1.02%      |         |
| SQL Workspace Generation     | 1 second, 194 ms, 751 µs and 326 ns           | 1.79%      |         |
| Formatting Workspace         | 202 ms, 643 µs and 747 ns                     | 0.30%      |         |
| Checking Generated Workspace | 1 minute, 1 second, 342 ms, 254 µs and 219 ns | 91.91%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 175 ms, 373 µs and 513 ns (98.38% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 15 ms, 751 µs and 632 ns            | 1.32%      |         |
| writing_crate_lib  | 1 second, 175 ms, 373 µs and 513 ns | 98.38%     |         |
| workspace_toml     | 3 ms, 559 µs and 89 ns              | 0.30%      |         |
| workspace_rustfmt  | 67 µs and 92 ns                     | 0.01%      |         |

![Plot](emi_codegen.png)
