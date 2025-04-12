# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took 24 seconds (50.00% of all time).

| name                            | time       | percentage | comment    |
|---------------------------------|------------|------------|------------|
| Checking Migrations Directory   | now        | 0.00%      | Unchanged. |
| Creating Font Awesome Icons CSV | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV  | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs       | 19 seconds | 39.58%     | Unchanged. |
| Executing Migrations            | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL              | now        | 0.00%      | Unchanged. |
| Checking Constraints            | now        | 6.25%      | Unchanged. |
| Code generation                 | 24 seconds | 50.00%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was 24 seconds.
The slowest task was `Generate Web Common Traits` which took 12 seconds (50.00% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Retrieving tables          | now        | 0.00%      | Unchanged. |
| Generating Diesel code     | now        | 4.17%      | Unchanged. |
| Generate Structs           | now        | 41.67%     | Unchanged. |
| Generate Web Common Traits | 12 seconds | 50.00%     | Unchanged. |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment    |
|--------------------------------------------------------|------|------------|------------|
| Generating tables schema                               | now  | 0.00%      | Unchanged. |
| Generating types schema                                | now  | 0.00%      | Unchanged. |
| Generating allow tables to appear in same query schema | now  | 100.00%    | Unchanged. |

### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took now (100.00% of all time).

| name                   | time | percentage | comment    |
|------------------------|------|------------|------------|
| Generate Types Structs | now  | 0.00%      | Unchanged. |
| Generate Table Structs | now  | 100.00%    | Unchanged. |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 12 seconds.
The slowest task was `Generate Table Traits` which took 12 seconds (100.00% of all time).

| name                  | time       | percentage | comment    |
|-----------------------|------------|------------|------------|
| Generate Types Traits | now        | 0.00%      | Unchanged. |
| Generate Table Traits | 12 seconds | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 12 seconds.
The slowest task was `Generate Insertable Traits` which took now (41.67% of all time).

| name                       | time | percentage | comment    |
|----------------------------|------|------------|------------|
| Generate Deletable Traits  | now  | 8.33%      | Unchanged. |
| Generate Loadable Traits   | now  | 0.00%      | Unchanged. |
| Generate Foreign Traits    | now  | 16.67%     | Unchanged. |
| Generate Insertable Traits | now  | 41.67%     | Unchanged. |
| Generate Updatable Traits  | now  | 16.67%     | Unchanged. |

![Plot](time_requirements_report.png)
