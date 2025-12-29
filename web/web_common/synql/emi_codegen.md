# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 23 seconds.
The slowest task was `Checking Generated Workspace` which took 18 seconds, 272 ms, 317 µs and 582 ns (76.74% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 457 ms, 599 µs and 838 ns  | 14.52%     |         |
| Schema Validation            | 683 ms, 202 µs and 796 ns             | 2.87%      |         |
| SQL Workspace Generation     | 1 second, 191 ms, 405 µs and 242 ns   | 5.00%      |         |
| Formatting Workspace         | 205 ms, 341 µs and 369 ns             | 0.86%      |         |
| Checking Generated Workspace | 18 seconds, 272 ms, 317 µs and 582 ns | 76.74%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 172 ms, 180 µs and 366 ns (98.39% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 15 ms, 593 µs and 459 ns            | 1.31%      |         |
| writing_crate_lib  | 1 second, 172 ms, 180 µs and 366 ns | 98.39%     |         |
| workspace_toml     | 3 ms, 565 µs and 77 ns              | 0.30%      |         |
| workspace_rustfmt  | 66 µs and 340 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
