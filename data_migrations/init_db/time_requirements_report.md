# Time Report for Test Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB` which took 6 minutes, 49 ms, 705 µs and 624 ns (99.17% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Booting up Docker | 3 seconds, 281 ms, 463 µs and 786 ns | 0.83%      |         |
| Init DB           | 6 minutes, 49 ms, 705 µs and 624 ns  | 99.17%     |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 49 ms, 573 µs and 221 ns (100.00% of all time).

| name                | time                                | percentage | comment |
|---------------------|-------------------------------------|------------|---------|
| Retrieve CSVs       | 132 µs and 403 ns                   | 0.00%      |         |
| Init DB Transaction | 6 minutes, 49 ms, 573 µs and 221 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 53 seconds, 652 ms, 815 µs and 436 ns (98.06% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 713 ms, 965 µs and 895 ns             | 1.39%      |         |
| Initialize Migrations         | 682 ms, 791 µs and 890 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 53 seconds, 652 ms, 815 µs and 436 ns | 98.06%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 44 seconds, 813 ms, 515 µs and 64 ns (97.45% of all time).

| name                                               | time                                            | percentage | comment |
|----------------------------------------------------|-------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 44 seconds, 813 ms, 515 µs and 64 ns | 97.45%     |         |
| Check constraints in schema 'public'               | 8 seconds, 246 ms, 247 µs and 264 ns            | 2.27%      |         |
| Procedure and procedure template check constraints | 593 ms, 53 µs and 108 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 143 ms, 594 µs and 150 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 143 ms, 594 µs and 150 ns | 100.00%    |         |
| Lowercase column and table names         | 85 ms, 820 µs and 389 ns             | 0.00%      |         |
| Standard column names and types          | 9 ms, 321 µs and 129 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 6 ms, 62 µs and 373 ns               | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 449 µs and 223 ns              | 0.00%      |         |

![Plot](time_requirements_report.png)
