# Time Report for EMI Workspace Generation Test

The total time spent on all tasks was 32 seconds.
The slowest task was `Checking Generated Workspace` which took 24 seconds, 636 ms, 913 µs and 17 ns (76.21% of all time).

| name                         | time                                 | percentage | comment |
|------------------------------|--------------------------------------|------------|---------|
| Database Parsing             | 2 seconds, 864 ms, 153 µs and 564 ns | 8.86%      |         |
| SQL Workspace Generation     | 3 seconds, 235 ms, 727 µs and 785 ns | 10.01%     |         |
| Formatting Workspace         | 1 second, 592 ms, 235 µs and 930 ns  | 4.93%      |         |
| Checking Generated Workspace | 24 seconds, 636 ms, 913 µs and 17 ns | 76.21%     |         |

## Time Report for SQL Workspace Generation

The total time spent on all tasks was now.
The slowest task was `workspace_write_to_disk` which took 1 second, 158 ms, 883 µs and 255 ns (35.82% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| schema_macro                  | 7 ms, 733 µs and 522 ns             | 0.24%      |         |
| model                         | 291 ms, 181 µs and 93 ns            | 9.00%      |         |
| relations_trait               | 16 ms, 208 µs and 467 ns            | 0.50%      |         |
| attributes                    | 273 ms, 277 µs and 301 ns           | 8.45%      |         |
| value_settable_trait          | 5 ms, 688 µs and 141 ns             | 0.18%      |         |
| insertable_key_settable_trait | 10 ms, 387 µs and 156 ns            | 0.32%      |         |
| insertable                    | 482 ms, 979 µs and 683 ns           | 14.93%     |         |
| buildable                     | 555 ms, 734 µs and 911 ns           | 17.17%     |         |
| extension_attributes          | 153 ms, 171 µs and 90 ns            | 4.73%      |         |
| workspace_write_to_disk       | 1 second, 158 ms, 883 µs and 255 ns | 35.82%     |         |
| workspace_toml                | 280 ms, 196 µs and 61 ns            | 8.66%      |         |
| workspace_rustfmt             | 287 µs and 105 ns                   | 0.01%      |         |

![Plot](emi_codegen.png)
