# Time Report for Test Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB` which took 6 minutes, 39 seconds, 795 ms, 304 µs and 244 ns (99.01% of all time).

| name              | time                                             | percentage | comment |
|-------------------|--------------------------------------------------|------------|---------|
| Booting up Docker | 3 seconds, 249 ms, 236 µs and 890 ns             | 0.74%      |         |
| Init DB           | 6 minutes, 39 seconds, 795 ms, 304 µs and 244 ns | 99.01%     |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 39 seconds, 795 ms, 124 µs and 580 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 179 µs and 664 ns                                | 0.00%      |         |
| Init DB Transaction | 6 minutes, 39 seconds, 795 ms, 124 µs and 580 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 6 minutes, 33 seconds, 334 ms, 526 µs and 705 ns (98.50% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 768 ms, 43 µs and 163 ns              | 1.25%      |         |
| Initialize Migrations         | 692 ms, 554 µs and 712 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 6 minutes, 33 seconds, 334 ms, 526 µs and 705 ns | 98.50%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 6 minutes.
The slowest task was `Procedure and procedure template alignment` which took 6 minutes, 24 seconds, 233 ms, 271 µs and 742 ns (97.71% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 6 minutes, 24 seconds, 233 ms, 271 µs and 742 ns | 97.71%     |         |
| Check constraints in schema 'public'               | 8 seconds, 454 ms, 739 µs and 743 ns             | 2.04%      |         |
| Procedure and procedure template check constraints | 646 ms, 515 µs and 220 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 350 ms, 127 µs and 910 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 350 ms, 127 µs and 910 ns | 100.00%    |         |
| Lowercase column and table names         | 86 ms, 827 µs and 255 ns             | 0.00%      |         |
| Standard column names and types          | 9 ms, 70 µs and 838 ns               | 0.00%      |         |
| Not-null constraints on standard columns | 6 ms, 489 µs and 62 ns               | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 224 µs and 678 ns              | 0.00%      |         |

![Plot](time_requirements_report.png)
