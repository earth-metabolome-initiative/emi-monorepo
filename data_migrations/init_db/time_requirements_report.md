# Time Report for Test Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 37 seconds, 851 ms, 389 µs and 62 ns (96.44% of all time).

| name              | time                                            | percentage | comment |
|-------------------|-------------------------------------------------|------------|---------|
| Booting up Docker | 7 seconds, 731 ms, 293 µs and 120 ns            | 3.11%      |         |
| Init DB           | 3 minutes, 37 seconds, 851 ms, 389 µs and 62 ns | 96.44%     |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 37 seconds, 851 ms, 284 µs and 904 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 104 µs and 158 ns                                | 0.00%      |         |
| Init DB Transaction | 3 minutes, 37 seconds, 851 ms, 284 µs and 904 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 3 minutes, 33 seconds, 730 ms, 414 µs and 482 ns (98.16% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 638 ms, 429 µs and 112 ns             | 1.38%      |         |
| Initialize Migrations         | 482 ms, 441 µs and 310 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 3 minutes, 33 seconds, 730 ms, 414 µs and 482 ns | 98.16%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 3 minutes.
The slowest task was `Procedure and procedure template alignment` which took 3 minutes, 27 seconds, 629 ms, 340 µs and 589 ns (97.18% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 3 minutes, 27 seconds, 629 ms, 340 µs and 589 ns | 97.18%     |         |
| Check constraints in schema 'public'               | 5 seconds, 664 ms, 206 µs and 913 ns             | 2.35%      |         |
| Procedure and procedure template check constraints | 436 ms, 866 µs and 980 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 5 seconds, 599 ms, 125 µs and 140 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 5 seconds, 599 ms, 125 µs and 140 ns | 100.00%    |         |
| Lowercase column and table names         | 50 ms, 602 µs and 999 ns             | 0.00%      |         |
| Standard column names and types          | 8 ms, 260 µs and 414 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 5 ms, 144 µs and 243 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 74 µs and 117 ns               | 0.00%      |         |

![Plot](time_requirements_report.png)
