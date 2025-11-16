# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 28 seconds.
The slowest task was `Checking Generated Workspace` which took 18 seconds, 109 ms, 549 µs and 9 ns (63.56% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 638 ms, 484 µs and 431 ns | 12.77%     |         |
| Schema Validation            | 978 ms, 692 µs and 359 ns            | 3.43%      |         |
| SQL Workspace Generation     | 3 seconds, 24 ms, 838 µs and 273 ns  | 10.62%     |         |
| Formatting Workspace         | 2 seconds, 742 ms, 542 µs and 646 ns | 9.62%      |         |
| Checking Generated Workspace | 18 seconds, 109 ms, 549 µs and 9 ns  | 63.56%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 826 ms, 936 µs and 147 ns (60.40% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 17 ms, 321 µs and 929 ns            | 0.57%      |         |
| model                         | 587 ms, 769 µs and 703 ns           | 19.43%     |         |
| transitive_extension_trait    | 5 ms, 276 µs and 236 ns             | 0.17%      |         |
| relations_trait               | 14 ms, 805 µs and 578 ns            | 0.49%      |         |
| attributes                    | 17 ms, 659 µs and 587 ns            | 0.58%      |         |
| value_settable_trait          | 5 ms, 278 µs and 805 ns             | 0.17%      |         |
| insertable_key_settable_trait | 8 ms, 741 µs and 864 ns             | 0.29%      |         |
| buildable_key_settable_trait  | 25 ms, 283 µs and 267 ns            | 0.84%      |         |
| insertable                    | 30 ms, 305 µs and 147 ns            | 1.00%      |         |
| buildable                     | 213 ms, 467 µs and 70 ns            | 7.06%      |         |
| extension_attributes          | 12 ms, 346 µs and 995 ns            | 0.41%      |         |
| workspace_write_to_disk       | 1 second, 826 ms, 936 µs and 147 ns | 60.40%     |         |
| workspace_toml                | 259 ms, 535 µs and 277 ns           | 8.58%      |         |
| workspace_rustfmt             | 110 µs and 668 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
