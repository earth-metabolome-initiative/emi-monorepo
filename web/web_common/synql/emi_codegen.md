# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 26 seconds.
The slowest task was `Checking Generated Workspace` which took 17 seconds, 81 ms, 329 µs and 396 ns (65.36% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 447 ms, 627 µs and 403 ns | 13.19%     |         |
| SQL Workspace Generation     | 3 seconds, 156 ms, 490 µs and 115 ns | 12.08%     |         |
| Formatting Workspace         | 2 seconds, 447 ms, 175 µs and 653 ns | 9.36%      |         |
| Checking Generated Workspace | 17 seconds, 81 ms, 329 µs and 396 ns | 65.36%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 452 ms, 430 µs and 117 ns (77.69% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 16 ms, 708 µs and 867 ns             | 0.53%      |         |
| model                         | 81 ms, 308 µs and 331 ns             | 2.58%      |         |
| relations_trait               | 12 ms, 952 µs and 528 ns             | 0.41%      |         |
| attributes                    | 18 ms, 738 µs and 936 ns             | 0.59%      |         |
| value_settable_trait          | 5 ms, 558 µs and 39 ns               | 0.18%      |         |
| insertable_key_settable_trait | 8 ms, 898 µs and 298 ns              | 0.28%      |         |
| buildable_key_settable_trait  | 49 ms, 358 µs and 293 ns             | 1.56%      |         |
| insertable                    | 28 ms, 490 µs and 783 ns             | 0.90%      |         |
| buildable                     | 209 ms, 253 µs and 198 ns            | 6.63%      |         |
| extension_attributes          | 12 ms, 60 µs and 514 ns              | 0.38%      |         |
| workspace_write_to_disk       | 2 seconds, 452 ms, 430 µs and 117 ns | 77.69%     |         |
| workspace_toml                | 260 ms, 618 µs and 88 ns             | 8.26%      |         |
| workspace_rustfmt             | 114 µs and 123 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
