# Time Report for Test Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 23 seconds, 77 ms, 200 µs and 664 ns (99.08% of all time).

| name              | time                                            | percentage | comment |
|-------------------|-------------------------------------------------|------------|---------|
| Booting up Docker | 3 seconds, 190 ms, 293 µs and 795 ns            | 0.92%      |         |
| Init DB           | 5 minutes, 23 seconds, 77 ms, 200 µs and 664 ns | 99.08%     |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 23 seconds, 76 ms, 911 µs and 457 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 289 µs and 207 ns                               | 0.00%      |         |
| Init DB Transaction | 5 minutes, 23 seconds, 76 ms, 911 µs and 457 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 16 seconds, 765 ms, 363 µs and 493 ns (97.83% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 655 ms, 765 µs and 142 ns             | 1.55%      |         |
| Initialize Migrations         | 655 ms, 782 µs and 822 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 16 seconds, 765 ms, 363 µs and 493 ns | 97.83%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 7 seconds, 869 ms, 417 µs and 450 ns (97.15% of all time).

| name                                               | time                                            | percentage | comment |
|----------------------------------------------------|-------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 7 seconds, 869 ms, 417 µs and 450 ns | 97.15%     |         |
| Check constraints in schema 'public'               | 7 seconds, 918 ms, 646 µs and 82 ns             | 2.22%      |         |
| Procedure and procedure template check constraints | 977 ms, 299 µs and 961 ns                       | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 7 seconds, 803 ms, 93 µs and 684 ns (100.00% of all time).

| name                                     | time                                | percentage | comment |
|------------------------------------------|-------------------------------------|------------|---------|
| Compatible foreign type constraints      | 7 seconds, 803 ms, 93 µs and 684 ns | 100.00%    |         |
| Lowercase column and table names         | 84 ms, 502 µs and 399 ns            | 0.00%      |         |
| Standard column names and types          | 16 ms, 909 µs and 786 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 11 ms, 201 µs and 716 ns            | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 938 µs and 497 ns             | 0.00%      |         |

![Plot](time_requirements_report.png)
