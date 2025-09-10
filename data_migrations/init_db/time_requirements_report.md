# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 2 minutes, 53 seconds, 718 ms, 321 µs and 653 ns (94.54% of all time).

| name              | time                                             | percentage | comment |
|-------------------|--------------------------------------------------|------------|---------|
| Booting up Docker | 9 seconds, 462 ms, 64 µs and 659 ns              | 4.92%      |         |
| Init DB           | 2 minutes, 53 seconds, 718 ms, 321 µs and 653 ns | 94.54%     |         |

## Time Report for Init DB

The total time spent on all tasks was 2 minutes.
The slowest task was `Init DB Transaction` which took 2 minutes, 53 seconds, 718 ms, 203 µs and 333 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 118 µs and 320 ns                                | 0.00%      |         |
| Init DB Transaction | 2 minutes, 53 seconds, 718 ms, 203 µs and 333 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 2 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 49 seconds, 574 ms, 825 µs and 420 ns (97.69% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 663 ms, 275 µs and 795 ns             | 1.73%      |         |
| Initialize Migrations         | 480 ms, 102 µs and 118 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 49 seconds, 574 ms, 825 µs and 420 ns | 97.69%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 44 seconds, 563 ms, 567 µs and 495 ns (97.04% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 44 seconds, 563 ms, 567 µs and 495 ns | 97.04%     |         |
| Check constraints in schema 'public'               | 4 seconds, 644 ms, 653 µs and 877 ns             | 2.37%      |         |
| Procedure and procedure template check constraints | 366 ms, 604 µs and 48 ns                         | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 583 ms, 813 µs and 325 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 583 ms, 813 µs and 325 ns | 100.00%    |         |
| Lowercase column and table names         | 48 ms, 817 µs and 228 ns             | 0.00%      |         |
| Standard column names and types          | 6 ms, 580 µs and 484 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 4 ms, 382 µs and 583 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 60 µs and 257 ns               | 0.00%      |         |

![Plot](time_requirements_report.png)
