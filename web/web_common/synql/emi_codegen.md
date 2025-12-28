# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 710 ms, 37 µs and 495 ns (76.49% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 439 ms, 382 µs and 472 ns | 14.85%     |         |
| Schema Validation            | 677 ms, 667 µs and 193 ns            | 2.93%      |         |
| SQL Workspace Generation     | 1 second, 123 ms, 892 µs and 550 ns  | 4.85%      |         |
| Formatting Workspace         | 203 ms, 189 µs and 193 ns            | 0.88%      |         |
| Checking Generated Workspace | 17 seconds, 710 ms, 37 µs and 495 ns | 76.49%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 107 ms, 75 µs and 330 ns (98.50% of all time).

| name               | time                               | percentage | comment |
|--------------------|------------------------------------|------------|---------|
| writing_crate_toml | 13 ms, 842 µs and 230 ns           | 1.23%      |         |
| writing_crate_lib  | 1 second, 107 ms, 75 µs and 330 ns | 98.50%     |         |
| workspace_toml     | 2 ms, 913 µs and 948 ns            | 0.26%      |         |
| workspace_rustfmt  | 61 µs and 42 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
