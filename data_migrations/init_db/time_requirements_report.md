# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 16 seconds, 697 ms, 822 µs and 35 ns (95.61% of all time).

| name              | time                                            | percentage | comment |
|-------------------|-------------------------------------------------|------------|---------|
| Booting up Docker | 8 seconds, 433 ms, 312 µs and 935 ns            | 3.90%      |         |
| Init DB           | 3 minutes, 16 seconds, 697 ms, 822 µs and 35 ns | 95.61%     |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 16 seconds, 697 ms, 728 µs and 93 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 93 µs and 942 ns                                | 0.00%      |         |
| Init DB Transaction | 3 minutes, 16 seconds, 697 ms, 728 µs and 93 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 3 minutes, 12 seconds, 578 ms, 96 µs and 727 ns (97.96% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 634 ms, 657 µs and 939 ns            | 1.53%      |         |
| Initialize Migrations         | 484 ms, 973 µs and 427 ns                       | 0.00%      |         |
| Consistency Constraint Checks | 3 minutes, 12 seconds, 578 ms, 96 µs and 727 ns | 97.96%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 3 minutes.
The slowest task was `Procedure and procedure template alignment` which took 3 minutes, 6 seconds, 936 ms, 566 µs and 596 ns (96.88% of all time).

| name                                               | time                                            | percentage | comment |
|----------------------------------------------------|-------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 3 minutes, 6 seconds, 936 ms, 566 µs and 596 ns | 96.88%     |         |
| Check constraints in schema 'public'               | 5 seconds, 243 ms, 26 µs and 501 ns             | 2.60%      |         |
| Procedure and procedure template check constraints | 398 ms, 503 µs and 630 ns                       | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 5 seconds, 177 ms, 176 µs and 885 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 5 seconds, 177 ms, 176 µs and 885 ns | 100.00%    |         |
| Lowercase column and table names         | 54 ms, 1 µs and 811 ns               | 0.00%      |         |
| Standard column names and types          | 6 ms, 388 µs and 296 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 4 ms, 436 µs and 690 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 22 µs and 819 ns               | 0.00%      |         |

![Plot](time_requirements_report.png)
