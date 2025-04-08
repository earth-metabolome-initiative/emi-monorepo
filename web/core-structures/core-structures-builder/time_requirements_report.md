# Time Report for Building Core Structures

The total time spent on all tasks was 43 seconds.
The slowest task was `Code generation` which took 21 seconds (48.84% of all time).

| name                            | time       | percentage | comment    |
|---------------------------------|------------|------------|------------|
| Checking Migrations Directory   | now        | 0.00%      | Unchanged. |
| Creating Font Awesome Icons CSV | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV  | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs       | 19 seconds | 44.19%     | Unchanged. |
| Executing Migrations            | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL              | now        | 0.00%      | Unchanged. |
| Checking Constraints            | now        | 6.98%      | Unchanged. |
| Code generation                 | 21 seconds | 48.84%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was 21 seconds.
The slowest task was `Generate Web Common Traits` which took 11 seconds (52.38% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Retrieving tables          | now        | 0.00%      | Unchanged. |
| Generating Diesel code     | now        | 4.76%      | Unchanged. |
| Generate Structs           | now        | 38.10%     | Unchanged. |
| Generate Web Common Traits | 11 seconds | 52.38%     | Unchanged. |

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

The total time spent on all tasks was 11 seconds.
The slowest task was `Generate Table Traits` which took 11 seconds (100.00% of all time).

| name                  | time       | percentage | comment    |
|-----------------------|------------|------------|------------|
| Generate Types Traits | now        | 0.00%      | Unchanged. |
| Generate Table Traits | 11 seconds | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 11 seconds.
The slowest task was `Generate Insertable Traits` which took now (45.45% of all time).

| name                       | time | percentage | comment    |
|----------------------------|------|------------|------------|
| Generate Deletable Traits  | now  | 0.00%      | Unchanged. |
| Generate Loadable Traits   | now  | 0.00%      | Unchanged. |
| Generate Foreign Traits    | now  | 18.18%     | Unchanged. |
| Generate Insertable Traits | now  | 45.45%     | Unchanged. |
| Generate Updatable Traits  | now  | 18.18%     | Unchanged. |

![Plot](time_requirements_report.png)
