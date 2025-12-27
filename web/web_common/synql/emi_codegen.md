# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 39 seconds.
The slowest task was `Checking Generated Workspace` which took 33 seconds, 837 ms, 28 µs and 4 ns (85.47% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 445 ms, 370 µs and 714 ns | 8.70%      |         |
| Schema Validation            | 650 ms, 108 µs and 440 ns            | 1.64%      |         |
| SQL Workspace Generation     | 1 second, 446 ms, 55 µs and 851 ns   | 3.65%      |         |
| Formatting Workspace         | 211 ms, 451 µs and 815 ns            | 0.53%      |         |
| Checking Generated Workspace | 33 seconds, 837 ms, 28 µs and 4 ns   | 85.47%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 422 ms, 778 µs and 349 ns (98.39% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 18 ms, 792 µs and 999 ns            | 1.30%      |         |
| writing_crate_lib  | 1 second, 422 ms, 778 µs and 349 ns | 98.39%     |         |
| workspace_toml     | 4 ms, 402 µs and 399 ns             | 0.30%      |         |
| workspace_rustfmt  | 82 µs and 104 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
