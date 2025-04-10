# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took 43 seconds (59.72% of all time).

| name                            | time       | percentage | comment    |
|---------------------------------|------------|------------|------------|
| Checking Migrations Directory   | now        | 0.00%      | Unchanged. |
| Creating Font Awesome Icons CSV | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV  | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs       | 25 seconds | 34.72%     | Unchanged. |
| Executing Migrations            | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL              | now        | 0.00%      | Unchanged. |
| Checking Constraints            | now        | 2.78%      | Unchanged. |
| Code generation                 | 43 seconds | 59.72%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was 43 seconds.
The slowest task was `Generate Structs` which took 20 seconds (46.51% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Retrieving tables          | now        | 0.00%      | Unchanged. |
| Generating Diesel code     | now        | 6.98%      | Unchanged. |
| Generate Structs           | 20 seconds | 46.51%     | Unchanged. |
| Generate Web Common Traits | 19 seconds | 44.19%     | Unchanged. |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment    |
|--------------------------------------------------------|------|------------|------------|
| Generating tables schema                               | now  | 0.00%      | Unchanged. |
| Generating types schema                                | now  | 0.00%      | Unchanged. |
| Generating allow tables to appear in same query schema | now  | 100.00%    | Unchanged. |

### Time Report for Generate Structs

The total time spent on all tasks was 20 seconds.
The slowest task was `Generate Table Structs` which took 20 seconds (100.00% of all time).

| name                   | time       | percentage | comment    |
|------------------------|------------|------------|------------|
| Generate Types Structs | now        | 0.00%      | Unchanged. |
| Generate Table Structs | 20 seconds | 100.00%    | Unchanged. |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 19 seconds.
The slowest task was `Generate Table Traits` which took 19 seconds (100.00% of all time).

| name                  | time       | percentage | comment    |
|-----------------------|------------|------------|------------|
| Generate Types Traits | now        | 0.00%      | Unchanged. |
| Generate Table Traits | 19 seconds | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 19 seconds.
The slowest task was `Generate Insertable Traits` which took now (47.37% of all time).

| name                       | time | percentage | comment    |
|----------------------------|------|------------|------------|
| Generate Deletable Traits  | now  | 5.26%      | Unchanged. |
| Generate Loadable Traits   | now  | 0.00%      | Unchanged. |
| Generate Foreign Traits    | now  | 21.05%     | Unchanged. |
| Generate Insertable Traits | now  | 47.37%     | Unchanged. |
| Generate Updatable Traits  | now  | 10.53%     | Unchanged. |

![Plot](time_requirements_report.png)
