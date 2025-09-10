# Time Report for Test Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 14 seconds, 78 ms, 319 µs and 611 ns (99.37% of all time).

| name              | time                                            | percentage | comment |
|-------------------|-------------------------------------------------|------------|---------|
| Booting up Docker | 2 seconds, 717 ms, 944 µs and 995 ns            | 0.63%      |         |
| Init DB           | 5 minutes, 14 seconds, 78 ms, 319 µs and 611 ns | 99.37%     |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 14 seconds, 78 ms, 116 µs and 427 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 203 µs and 184 ns                               | 0.00%      |         |
| Init DB Transaction | 5 minutes, 14 seconds, 78 ms, 116 µs and 427 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 7 seconds, 586 ms, 280 µs and 95 ns (97.77% of all time).

| name                          | time                                           | percentage | comment |
|-------------------------------|------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 779 ms, 378 µs and 695 ns           | 1.59%      |         |
| Initialize Migrations         | 712 ms, 457 µs and 637 ns                      | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 7 seconds, 586 ms, 280 µs and 95 ns | 97.77%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 4 minutes, 59 seconds, 219 ms, 247 µs and 413 ns (97.39% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 4 minutes, 59 seconds, 219 ms, 247 µs and 413 ns | 97.39%     |         |
| Check constraints in schema 'public'               | 7 seconds, 389 ms, 604 µs and 520 ns             | 2.28%      |         |
| Procedure and procedure template check constraints | 977 ms, 428 µs and 162 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 7 seconds, 275 ms, 954 µs and 318 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 7 seconds, 275 ms, 954 µs and 318 ns | 100.00%    |         |
| Lowercase column and table names         | 83 ms, 188 µs and 391 ns             | 0.00%      |         |
| Standard column names and types          | 16 ms, 235 µs and 403 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 11 ms, 248 µs and 722 ns             | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 977 µs and 686 ns              | 0.00%      |         |

![Plot](time_requirements_report.png)
