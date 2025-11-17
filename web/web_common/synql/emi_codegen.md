# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 29 seconds.
The slowest task was `Checking Generated Workspace` which took 19 seconds, 307 ms, 101 µs and 840 ns (66.03% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 375 ms, 857 µs and 890 ns  | 11.55%     |         |
| Schema Validation            | 959 ms, 385 µs and 503 ns             | 3.28%      |         |
| SQL Workspace Generation     | 2 seconds, 840 ms, 631 µs and 796 ns  | 9.72%      |         |
| Formatting Workspace         | 2 seconds, 755 ms, 423 µs and 66 ns   | 9.42%      |         |
| Checking Generated Workspace | 19 seconds, 307 ms, 101 µs and 840 ns | 66.03%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 638 ms, 226 µs and 469 ns (57.67% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 25 ms, 622 µs and 625 ns            | 0.90%      |         |
| model                         | 560 ms, 94 µs and 836 ns            | 19.72%     |         |
| transitive_extension_trait    | 5 ms, 29 µs and 611 ns              | 0.18%      |         |
| relations_trait               | 14 ms, 262 µs and 871 ns            | 0.50%      |         |
| attributes                    | 17 ms, 859 µs and 41 ns             | 0.63%      |         |
| value_settable_trait          | 5 ms, 740 µs and 499 ns             | 0.20%      |         |
| insertable_key_settable_trait | 8 ms, 675 µs and 301 ns             | 0.31%      |         |
| buildable_key_settable_trait  | 24 ms, 659 µs and 500 ns            | 0.87%      |         |
| insertable                    | 30 ms, 862 µs and 502 ns            | 1.09%      |         |
| buildable                     | 207 ms, 786 µs and 939 ns           | 7.31%      |         |
| extension_attributes          | 11 ms, 928 µs and 641 ns            | 0.42%      |         |
| workspace_write_to_disk       | 1 second, 638 ms, 226 µs and 469 ns | 57.67%     |         |
| workspace_toml                | 289 ms, 763 µs and 691 ns           | 10.20%     |         |
| workspace_rustfmt             | 119 µs and 270 ns                   | 0.00%      |         |

![Plot](emi_codegen.png)
