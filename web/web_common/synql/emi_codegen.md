# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was now.
The slowest task was `Database Parsing` which took 3 seconds, 414 ms, 681 µs and 170 ns (60.00% of all time).

| name                     | time                                 | percentage | comment |
|--------------------------|--------------------------------------|------------|---------|
| Database Parsing         | 3 seconds, 414 ms, 681 µs and 170 ns | 60.00%     |         |
| Codegen setup            | 175 µs and 15 ns                     | 0.00%      |         |
| SQL Workspace Generation | 1 second, 937 ms, 107 µs and 596 ns  | 20.00%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 69 ms, 141 µs and 472 ns (100.00% of all time).

| name                    | time                               | percentage | comment |
|-------------------------|------------------------------------|------------|---------|
| schema_macro            | 9 ms, 571 µs and 171 ns            | 0.00%      |         |
| model                   | 88 ms, 319 µs and 751 ns           | 0.00%      |         |
| relations_trait         | 11 ms, 882 µs and 933 ns           | 0.00%      |         |
| attributes              | 16 ms, 99 µs and 954 ns            | 0.00%      |         |
| value_settable_trait    | 4 ms, 745 µs and 971 ns            | 0.00%      |         |
| insertable              | 11 ms, 207 µs and 55 ns            | 0.00%      |         |
| buildable               | 28 ms, 169 µs and 507 ns           | 0.00%      |         |
| extension_attributes    | 10 ms, 612 µs and 187 ns           | 0.00%      |         |
| workspace_write_to_disk | 1 second, 69 ms, 141 µs and 472 ns | 100.00%    |         |
| workspace_toml          | 687 ms, 39 µs and 273 ns           | 0.00%      |         |
| workspace_rustfmt       | 318 µs and 322 ns                  | 0.00%      |         |

![Plot](emi_codegen.png)
