# Time Report for Test Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB` which took 6 minutes, 41 seconds, 146 ms, 949 µs and 180 ns (99.26% of all time).

| name              | time                                             | percentage | comment |
|-------------------|--------------------------------------------------|------------|---------|
| Booting up Docker | 3 seconds, 38 ms, 479 µs and 974 ns              | 0.74%      |         |
| Init DB           | 6 minutes, 41 seconds, 146 ms, 949 µs and 180 ns | 99.26%     |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 41 seconds, 146 ms, 799 µs and 266 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 149 µs and 914 ns                                | 0.00%      |         |
| Init DB Transaction | 6 minutes, 41 seconds, 146 ms, 799 µs and 266 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 6 minutes, 34 seconds, 669 ms, 212 µs and 843 ns (98.25% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 768 ms, 903 µs and 279 ns             | 1.25%      |         |
| Initialize Migrations         | 708 ms, 683 µs and 144 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 6 minutes, 34 seconds, 669 ms, 212 µs and 843 ns | 98.25%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 6 minutes.
The slowest task was `Procedure and procedure template alignment` which took 6 minutes, 25 seconds, 219 ms, 301 µs and 998 ns (97.72% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 6 minutes, 25 seconds, 219 ms, 301 µs and 998 ns | 97.72%     |         |
| Check constraints in schema 'public'               | 8 seconds, 369 ms, 69 µs and 472 ns              | 2.03%      |         |
| Procedure and procedure template check constraints | 1 second, 80 ms, 841 µs and 373 ns               | 0.25%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 257 ms, 797 µs and 658 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 257 ms, 797 µs and 658 ns | 100.00%    |         |
| Lowercase column and table names         | 84 ms, 473 µs and 699 ns             | 0.00%      |         |
| Standard column names and types          | 14 ms, 408 µs and 385 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 9 ms, 313 µs and 501 ns              | 0.00%      |         |
| Word deprecation constraints             | 3 ms, 76 µs and 229 ns               | 0.00%      |         |

![Plot](time_requirements_report.png)
