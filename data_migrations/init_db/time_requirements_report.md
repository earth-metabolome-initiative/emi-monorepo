# Time Report for Test Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 23 seconds, 880 ms, 680 µs and 701 ns (98.78% of all time).

| name              | time                                             | percentage | comment |
|-------------------|--------------------------------------------------|------------|---------|
| Booting up Docker | 3 seconds, 183 ms, 970 µs and 186 ns             | 0.92%      |         |
| Init DB           | 5 minutes, 23 seconds, 880 ms, 680 µs and 701 ns | 98.78%     |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 23 seconds, 880 ms, 379 µs and 464 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 301 µs and 237 ns                                | 0.00%      |         |
| Init DB Transaction | 5 minutes, 23 seconds, 880 ms, 379 µs and 464 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 17 seconds, 399 ms, 615 µs and 740 ns (98.14% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 811 ms, 924 µs and 546 ns             | 1.55%      |         |
| Initialize Migrations         | 668 ms, 839 µs and 178 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 17 seconds, 399 ms, 615 µs and 740 ns | 98.14%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 8 seconds, 597 ms, 197 µs and 179 ns (97.16% of all time).

| name                                               | time                                            | percentage | comment |
|----------------------------------------------------|-------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 8 seconds, 597 ms, 197 µs and 179 ns | 97.16%     |         |
| Check constraints in schema 'public'               | 7 seconds, 852 ms, 442 µs and 68 ns             | 2.21%      |         |
| Procedure and procedure template check constraints | 949 ms, 976 µs and 493 ns                       | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 7 seconds, 738 ms, 267 µs and 738 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 7 seconds, 738 ms, 267 µs and 738 ns | 100.00%    |         |
| Lowercase column and table names         | 82 ms, 256 µs and 63 ns              | 0.00%      |         |
| Standard column names and types          | 16 ms, 456 µs and 474 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 12 ms, 14 µs and 467 ns              | 0.00%      |         |
| Word deprecation constraints             | 3 ms, 447 µs and 326 ns              | 0.00%      |         |

![Plot](time_requirements_report.png)
