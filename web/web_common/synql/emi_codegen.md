# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 30 seconds.
The slowest task was `Checking Generated Workspace` which took 21 seconds, 939 ms, 89 µs and 738 ns (70.00% of all time).

| name                                        | time                                 | percentage | comment |
|---------------------------------------------|--------------------------------------|------------|---------|
| Database Parsing                            | 3 seconds, 647 ms, 685 µs and 227 ns | 10.00%     |         |
| SQL Workspace Generation                    | 3 seconds, 732 ms, 476 µs and 481 ns | 10.00%     |         |
| Formatting and Checking Generated Workspace | 1 second, 335 ms, 719 µs and 443 ns  | 3.33%      |         |
| Checking Generated Workspace                | 21 seconds, 939 ms, 89 µs and 738 ns | 70.00%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 44 ms, 639 µs and 127 ns (66.67% of all time).

| name                    | time                                | percentage | comment |
|-------------------------|-------------------------------------|------------|---------|
| schema_macro            | 11 ms, 308 µs and 832 ns            | 0.00%      |         |
| model                   | 86 ms, 711 µs and 353 ns            | 0.00%      |         |
| relations_trait         | 12 ms, 790 µs and 17 ns             | 0.00%      |         |
| attributes              | 17 ms, 604 µs and 557 ns            | 0.00%      |         |
| value_settable_trait    | 5 ms, 575 µs and 453 ns             | 0.00%      |         |
| insertable              | 14 ms, 301 µs and 335 ns            | 0.00%      |         |
| buildable               | 29 ms, 797 µs and 529 ns            | 0.00%      |         |
| extension_attributes    | 11 ms, 436 µs and 494 ns            | 0.00%      |         |
| workspace_write_to_disk | 2 seconds, 44 ms, 639 µs and 127 ns | 66.67%     |         |
| workspace_toml          | 1 second, 498 ms, 20 µs and 963 ns  | 33.33%     |         |
| workspace_rustfmt       | 290 µs and 821 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
