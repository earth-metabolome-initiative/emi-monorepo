# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 37 seconds.
The slowest task was `Checking Generated Workspace` which took 28 seconds, 567 ms, 41 µs and 303 ns (76.01% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 362 ms, 189 µs and 20 ns  | 8.95%      |         |
| SQL Workspace Generation     | 3 seconds, 201 ms, 949 µs and 106 ns | 8.52%      |         |
| Formatting Workspace         | 2 seconds, 454 ms, 546 µs and 240 ns | 6.53%      |         |
| Checking Generated Workspace | 28 seconds, 567 ms, 41 µs and 303 ns | 76.01%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 506 ms, 499 µs and 152 ns (78.28% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 6 ms, 747 µs and 219 ns              | 0.21%      |         |
| model                         | 81 ms, 821 µs and 346 ns             | 2.56%      |         |
| relations_trait               | 12 ms, 859 µs and 105 ns             | 0.40%      |         |
| attributes                    | 16 ms, 678 µs and 875 ns             | 0.52%      |         |
| value_settable_trait          | 5 ms, 515 µs and 980 ns              | 0.17%      |         |
| insertable_key_settable_trait | 8 ms, 791 µs and 294 ns              | 0.27%      |         |
| buildable_key_settable_trait  | 48 ms, 962 µs and 460 ns             | 1.53%      |         |
| insertable                    | 27 ms, 797 µs and 556 ns             | 0.87%      |         |
| buildable                     | 207 ms, 900 µs and 558 ns            | 6.49%      |         |
| extension_attributes          | 11 ms, 177 µs and 96 ns              | 0.35%      |         |
| workspace_write_to_disk       | 2 seconds, 506 ms, 499 µs and 152 ns | 78.28%     |         |
| workspace_toml                | 267 ms, 97 µs and 312 ns             | 8.34%      |         |
| workspace_rustfmt             | 101 µs and 153 ns                    | 0.00%      |         |

![Plot](emi_codegen.png)
