# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 2 seconds, 455 ms, 246 µs and 236 ns (95.29% of all time).

| name              | time                                            | percentage | comment |
|-------------------|-------------------------------------------------|------------|---------|
| Booting up Docker | 9 seconds, 107 ms, 281 µs and 454 ns            | 4.71%      |         |
| Init DB           | 3 minutes, 2 seconds, 455 ms, 246 µs and 236 ns | 95.29%     |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 2 seconds, 455 ms, 138 µs and 813 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 107 µs and 423 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 2 seconds, 455 ms, 138 µs and 813 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 58 seconds, 477 ms, 430 µs and 906 ns (97.80% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 532 ms, 645 µs and 809 ns             | 1.65%      |         |
| Initialize Migrations         | 445 ms, 62 µs and 98 ns                          | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 58 seconds, 477 ms, 430 µs and 906 ns | 97.80%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 53 seconds, 478 ms, 761 µs and 999 ns (97.19% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 53 seconds, 478 ms, 761 µs and 999 ns | 97.19%     |         |
| Check constraints in schema 'public'               | 4 seconds, 645 ms, 884 µs and 450 ns             | 2.25%      |         |
| Procedure and procedure template check constraints | 352 ms, 784 µs and 457 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 589 ms, 594 µs and 816 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 589 ms, 594 µs and 816 ns | 100.00%    |         |
| Lowercase column and table names         | 46 ms, 374 µs and 68 ns              | 0.00%      |         |
| Standard column names and types          | 5 ms, 392 µs and 456 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 3 ms, 566 µs and 501 ns              | 0.00%      |         |
| Word deprecation constraints             | 956 µs and 609 ns                    | 0.00%      |         |

![Plot](time_requirements_report.png)
