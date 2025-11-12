# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 37 seconds.
The slowest task was `Checking Generated Workspace` which took 27 seconds, 448 ms, 711 µs and 672 ns (73.58% of all time).

| name                         | time                                  | percentage | comment |
|------------------------------|---------------------------------------|------------|---------|
| Database Parsing             | 3 seconds, 381 ms, 99 µs and 360 ns   | 9.06%      |         |
| SQL Workspace Generation     | 4 seconds, 849 ms, 657 µs and 444 ns  | 13.00%     |         |
| Formatting Workspace         | 1 second, 624 ms, 950 µs and 851 ns   | 4.36%      |         |
| Checking Generated Workspace | 27 seconds, 448 ms, 711 µs and 672 ns | 73.58%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 2 seconds, 619 ms, 306 µs and 566 ns (54.01% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| schema_macro                  | 8 ms, 221 µs and 765 ns              | 0.17%      |         |
| model                         | 306 ms, 631 µs and 237 ns            | 6.32%      |         |
| relations_trait               | 17 ms, 890 µs and 647 ns             | 0.37%      |         |
| attributes                    | 301 ms, 602 µs and 980 ns            | 6.22%      |         |
| value_settable_trait          | 6 ms, 144 µs and 860 ns              | 0.13%      |         |
| insertable_key_settable_trait | 10 ms, 982 µs and 16 ns              | 0.23%      |         |
| insertable                    | 519 ms, 412 µs and 701 ns            | 10.71%     |         |
| buildable                     | 600 ms, 492 µs and 710 ns            | 12.38%     |         |
| extension_attributes          | 174 ms, 816 µs and 740 ns            | 3.60%      |         |
| workspace_write_to_disk       | 2 seconds, 619 ms, 306 µs and 566 ns | 54.01%     |         |
| workspace_toml                | 283 ms, 852 µs and 503 ns            | 5.85%      |         |
| workspace_rustfmt             | 302 µs and 719 ns                    | 0.01%      |         |

![Plot](emi_codegen.png)
