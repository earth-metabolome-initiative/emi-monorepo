# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 3 seconds, 29 ms, 947 µs and 783 ns (95.81% of all time).

| name              | time                                           | percentage | comment |
|-------------------|------------------------------------------------|------------|---------|
| Booting up Docker | 8 seconds, 371 ms, 104 µs and 844 ns           | 4.19%      |         |
| Init DB           | 3 minutes, 3 seconds, 29 ms, 947 µs and 783 ns | 95.81%     |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 3 seconds, 29 ms, 795 µs and 743 ns (100.00% of all time).

| name                | time                                           | percentage | comment |
|---------------------|------------------------------------------------|------------|---------|
| Retrieve CSVs       | 152 µs and 40 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 3 seconds, 29 ms, 795 µs and 743 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 59 seconds, 58 ms, 659 µs and 541 ns (97.81% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 533 ms, 508 µs and 361 ns            | 1.64%      |         |
| Initialize Migrations         | 437 ms, 627 µs and 841 ns                       | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 59 seconds, 58 ms, 659 µs and 541 ns | 97.81%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 53 seconds, 856 ms, 607 µs and 781 ns (96.65% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 53 seconds, 856 ms, 607 µs and 781 ns | 96.65%     |         |
| Check constraints in schema 'public'               | 4 seconds, 854 ms, 552 µs and 653 ns             | 2.23%      |         |
| Procedure and procedure template check constraints | 347 ms, 499 µs and 107 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 796 ms, 225 µs and 403 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 796 ms, 225 µs and 403 ns | 100.00%    |         |
| Lowercase column and table names         | 47 ms, 779 µs and 924 ns             | 0.00%      |         |
| Standard column names and types          | 5 ms, 513 µs and 191 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 3 ms, 920 µs and 658 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 113 µs and 477 ns              | 0.00%      |         |

![Plot](time_requirements_report.png)
