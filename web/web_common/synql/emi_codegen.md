# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 36 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 592 ms, 798 µs and 411 ns (75.04% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 343 ms, 394 µs and 3 ns    | 9.09%      |         |
| SQL Workspace Generation     | 3 seconds, 310 ms, 480 µs and 971 ns  | 9.00%      |         |
| Formatting Workspace         | 2 seconds, 523 ms, 382 µs and 822 ns  | 6.86%      |         |
| Checking Generated Workspace | 27 seconds, 592 ms, 798 µs and 411 ns | 75.04%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 596 ms, 96 µs and 793 ns (78.42% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 10 ms, 87 µs and 684 ns             | 0.30%      |         |
| model                         | 80 ms, 439 µs and 189 ns            | 2.43%      |         |
| relations_trait               | 13 ms, 12 µs and 629 ns             | 0.39%      |         |
| attributes                    | 16 ms, 837 µs and 820 ns            | 0.51%      |         |
| value_settable_trait          | 5 ms, 576 µs and 679 ns             | 0.17%      |         |
| insertable_key_settable_trait | 8 ms, 856 µs and 688 ns             | 0.27%      |         |
| buildable_key_settable_trait  | 49 ms, 98 µs and 495 ns             | 1.48%      |         |
| insertable                    | 28 ms, 420 µs and 803 ns            | 0.86%      |         |
| buildable                     | 209 ms, 269 µs and 703 ns           | 6.32%      |         |
| extension_attributes          | 11 ms, 41 µs and 253 ns             | 0.33%      |         |
| workspace_write_to_disk       | 2 seconds, 596 ms, 96 µs and 793 ns | 78.42%     |         |
| workspace_toml                | 281 ms, 606 µs and 478 ns           | 8.51%      |         |
| workspace_rustfmt             | 136 µs and 757 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
