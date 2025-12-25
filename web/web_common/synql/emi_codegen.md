# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 27 seconds.
The slowest task was `Checking Generated Workspace` which took 21 seconds, 199 ms, 111 µs and 597 ns (77.16% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 855 ms, 891 µs and 286 ns  | 14.04%     |         |
| Schema Validation            | 704 ms, 450 µs and 995 ns             | 2.56%      |         |
| SQL Workspace Generation     | 1 second, 510 ms, 430 µs and 75 ns    | 5.50%      |         |
| Formatting Workspace         | 203 ms, 6 µs and 42 ns                | 0.74%      |         |
| Checking Generated Workspace | 21 seconds, 199 ms, 111 µs and 597 ns | 77.16%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `writing_crate_lib` which took 1 second, 487 ms, 696 µs and 769 ns (98.49% of all time).

| name               | time                                | percentage | comment |
|--------------------|-------------------------------------|------------|---------|
| writing_crate_toml | 18 ms, 825 µs and 217 ns            | 1.25%      |         |
| writing_crate_lib  | 1 second, 487 ms, 696 µs and 769 ns | 98.49%     |         |
| workspace_toml     | 3 ms, 839 µs and 385 ns             | 0.25%      |         |
| workspace_rustfmt  | 68 µs and 704 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
