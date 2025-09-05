# Time Report for Test Init DB

The total time spent on all tasks was 2 minutes.
The slowest task was `Init DB` which took a minute (90.82% of all time).

| name              | time     | percentage | comment |
|-------------------|----------|------------|---------|
| Booting up Docker | now      | 9.18%      |         |
| Init DB           | a minute | 90.82%     |         |

## Time Report for Init DB

The total time spent on all tasks was a minute.
The slowest task was `Init DB Transaction` which took a minute (100.00% of all time).

| name                | time     | percentage | comment |
|---------------------|----------|------------|---------|
| Retrieve CSVs       | now      | 0.00%      |         |
| Init DB Transaction | a minute | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was a minute.
The slowest task was `Consistency Constraint Checks` which took a minute (94.38% of all time).

| name                          | time     | percentage | comment |
|-------------------------------|----------|------------|---------|
| Initialize CSVs               | now      | 3.37%      |         |
| Initialize Migrations         | now      | 0.00%      |         |
| Consistency Constraint Checks | a minute | 94.38%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was a minute.
The slowest task was `Procedure and procedure template alignment` which took a minute (95.24% of all time).

| name                                               | time     | percentage | comment |
|----------------------------------------------------|----------|------------|---------|
| Procedure and procedure template alignment         | a minute | 95.24%     |         |
| Check constraints in schema 'public'               | now      | 4.76%      |         |
| Procedure and procedure template check constraints | now      | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took now (75.00% of all time).

| name                                     | time | percentage | comment |
|------------------------------------------|------|------------|---------|
| Compatible foreign type constraints      | now  | 75.00%     |         |
| Lowercase column and table names         | now  | 0.00%      |         |
| Standard column names and types          | now  | 0.00%      |         |
| Not-null constraints on standard columns | now  | 0.00%      |         |
| Same-as constraint no cascade            | now  | 0.00%      |         |
| Word deprecation constraints             | now  | 0.00%      |         |

![Plot](time_requirements_report.png)
