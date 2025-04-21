# Time Report for Building Core Structures

The total time spent on all tasks was 2 minutes.
The slowest task was `Code generation` which took a minute (73.40% of all time).

| name                            | time       | percentage | comment    |
|---------------------------------|------------|------------|------------|
| Checking Migrations Directory   | now        | 0.00%      | Unchanged. |
| Creating Font Awesome Icons CSV | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV  | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs       | 17 seconds | 18.09%     | Unchanged. |
| Executing Migrations            | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL              | now        | 0.00%      | Unchanged. |
| Checking Constraints            | now        | 6.38%      | Unchanged. |
| Code generation                 | a minute   | 73.40%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Web Common Traits` which took 40 seconds (57.97% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Retrieving tables          | now        | 0.00%      | Unchanged. |
| Generating Diesel code     | now        | 4.35%      | Unchanged. |
| Generate Structs           | 25 seconds | 36.23%     | Unchanged. |
| Generate Web Common Traits | 40 seconds | 57.97%     | Unchanged. |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment    |
|--------------------------------------------------------|------|------------|------------|
| Generating tables schema                               | now  | 0.00%      | Unchanged. |
| Generating types schema                                | now  | 0.00%      | Unchanged. |
| Generating allow tables to appear in same query schema | now  | 100.00%    | Unchanged. |

### Time Report for Generate Structs

The total time spent on all tasks was 25 seconds.
The slowest task was `Generate Table Structs` which took 25 seconds (100.00% of all time).

| name                   | time       | percentage | comment    |
|------------------------|------------|------------|------------|
| Generate Types Structs | now        | 0.00%      | Unchanged. |
| Generate Table Structs | 25 seconds | 100.00%    | Unchanged. |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 40 seconds.
The slowest task was `Generate Table Traits` which took 40 seconds (100.00% of all time).

| name                  | time       | percentage | comment    |
|-----------------------|------------|------------|------------|
| Generate Types Traits | now        | 0.00%      | Unchanged. |
| Generate Table Traits | 40 seconds | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 40 seconds.
The slowest task was `Generate Insertable Traits` which took 22 seconds (55.00% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Generate Deletable Traits  | now        | 5.00%      | Unchanged. |
| Generate Loadable Traits   | now        | 2.50%      | Unchanged. |
| Generate Foreign Traits    | now        | 20.00%     | Unchanged. |
| Generate Insertable Traits | 22 seconds | 55.00%     | Unchanged. |
| Generate Updatable Traits  | now        | 10.00%     | Unchanged. |

![Plot](time_requirements_report.png)
