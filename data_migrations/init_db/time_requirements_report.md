# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 12 seconds, 163 ms, 285 µs and 387 ns (96.48% of all time).

| name              | time                                             | percentage | comment |
|-------------------|--------------------------------------------------|------------|---------|
| Booting up Docker | 7 seconds, 701 ms, 203 µs and 758 ns             | 3.52%      |         |
| Init DB           | 3 minutes, 12 seconds, 163 ms, 285 µs and 387 ns | 96.48%     |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 12 seconds, 163 ms, 166 µs and 127 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 119 µs and 260 ns                                | 0.00%      |         |
| Init DB Transaction | 3 minutes, 12 seconds, 163 ms, 166 µs and 127 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 3 minutes, 7 seconds, 965 ms, 203 µs and 689 ns (97.40% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 695 ms, 560 µs and 772 ns            | 1.56%      |         |
| Initialize Migrations         | 502 ms, 401 µs and 666 ns                       | 0.00%      |         |
| Consistency Constraint Checks | 3 minutes, 7 seconds, 965 ms, 203 µs and 689 ns | 97.40%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 3 minutes.
The slowest task was `Procedure and procedure template alignment` which took 3 minutes, 2 seconds, 554 ms, 48 µs and 465 ns (97.33% of all time).

| name                                               | time                                           | percentage | comment |
|----------------------------------------------------|------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 3 minutes, 2 seconds, 554 ms, 48 µs and 465 ns | 97.33%     |         |
| Check constraints in schema 'public'               | 5 seconds, 17 ms, 711 µs and 573 ns            | 2.67%      |         |
| Procedure and procedure template check constraints | 393 ms, 443 µs and 651 ns                      | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 956 ms, 354 µs and 763 ns (80.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 956 ms, 354 µs and 763 ns | 80.00%     |         |
| Lowercase column and table names         | 49 ms, 970 µs and 987 ns             | 0.00%      |         |
| Standard column names and types          | 6 ms, 179 µs and 894 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 4 ms, 180 µs and 144 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 25 µs and 785 ns               | 0.00%      |         |

![Plot](time_requirements_report.png)
