# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 24 seconds.
The slowest task was `Checking Generated Workspace` which took 18 seconds, 647 ms, 861 µs and 863 ns (77.46% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 331 ms, 622 µs and 829 ns  | 13.84%     |         |
| Schema Validation            | 682 ms, 413 µs and 557 ns             | 2.83%      |         |
| SQL Workspace Generation     | 1 second, 210 ms, 611 µs and 493 ns   | 5.03%      |         |
| Formatting Workspace         | 200 ms, 620 µs and 78 ns              | 0.83%      |         |
| Checking Generated Workspace | 18 seconds, 647 ms, 861 µs and 863 ns | 77.46%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 191 ms, 165 µs and 201 ns (98.39% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 15 ms, 791 µs and 17 ns             | 1.30%      |         |
| writing_crate_lib  | 1 second, 191 ms, 165 µs and 201 ns | 98.39%     |         |
| workspace_toml     | 3 ms, 589 µs and 95 ns              | 0.30%      |         |
| workspace_rustfmt  | 66 µs and 180 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
