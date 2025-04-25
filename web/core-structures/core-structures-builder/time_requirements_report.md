# Time Report for Building Core Structures

The total time spent on all tasks was 2 minutes.
The slowest task was `Code generation` which took 2 minutes (82.99% of all time).

| name                            | time       | percentage | comment    |
|---------------------------------|------------|------------|------------|
| Checking Migrations Directory   | now        | 0.00%      | Unchanged. |
| Creating Font Awesome Icons CSV | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV  | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs       | 17 seconds | 11.56%     | Unchanged. |
| Executing Migrations            | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL              | now        | 0.00%      | Unchanged. |
| Checking Constraints            | now        | 5.44%      | Unchanged. |
| Code generation                 | 2 minutes  | 82.99%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was 2 minutes.
The slowest task was `Generate Web Common Traits` which took a minute (54.10% of all time).

| name                       | time     | percentage | comment    |
|----------------------------|----------|------------|------------|
| Retrieving tables          | now      | 0.00%      | Unchanged. |
| Generating Diesel code     | now      | 3.28%      | Unchanged. |
| Generate Structs           | a minute | 40.98%     | Unchanged. |
| Generate Web Common Traits | a minute | 54.10%     | Unchanged. |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment    |
|--------------------------------------------------------|------|------------|------------|
| Generating tables schema                               | now  | 0.00%      | Unchanged. |
| Generating types schema                                | now  | 0.00%      | Unchanged. |
| Generating allow tables to appear in same query schema | now  | 100.00%    | Unchanged. |

### Time Report for Generate Structs

The total time spent on all tasks was a minute.
The slowest task was `Generate Table Structs` which took a minute (100.00% of all time).

| name                   | time     | percentage | comment    |
|------------------------|----------|------------|------------|
| Generate Types Structs | now      | 0.00%      | Unchanged. |
| Generate Table Structs | a minute | 100.00%    | Unchanged. |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was a minute.
The slowest task was `Generate Table Traits` which took a minute (100.00% of all time).

| name                  | time     | percentage | comment    |
|-----------------------|----------|------------|------------|
| Generate Types Traits | now      | 0.00%      | Unchanged. |
| Generate Table Traits | a minute | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was a minute.
The slowest task was `Generate Insertable Traits` which took 39 seconds (59.09% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Generate Deletable Traits  | now        | 4.55%      | Unchanged. |
| Generate Loadable Traits   | now        | 4.55%      | Unchanged. |
| Generate Foreign Traits    | 13 seconds | 19.70%     | Unchanged. |
| Generate Insertable Traits | 39 seconds | 59.09%     | Unchanged. |
| Generate Updatable Traits  | now        | 9.09%      | Unchanged. |

![Plot](time_requirements_report.png)
