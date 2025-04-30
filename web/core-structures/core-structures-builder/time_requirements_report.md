# Time Report for Building Core Structures

The total time spent on all tasks was 2 minutes.
The slowest task was `Code generation` which took a minute (74.73% of all time).

| name                           | time       | percentage | comment    |
|--------------------------------|------------|------------|------------|
| Checking Migrations Directory  | now        | 0.00%      | Unchanged. |
| Creating Taxonomical Ranks CSV | now        | 0.00%      | Unchanged. |
| Building Schema from CSVs      | 16 seconds | 17.58%     | Unchanged. |
| Executing Migrations           | now        | 0.00%      | Unchanged. |
| Executing Meta-SQL             | now        | 0.00%      | Unchanged. |
| Checking Constraints           | now        | 4.40%      | Unchanged. |
| Code generation                | a minute   | 74.73%     | Unchanged. |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Structs` which took 32 seconds (47.06% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Retrieving tables          | now        | 0.00%      | Unchanged. |
| Generating Diesel code     | now        | 13.24%     | Unchanged. |
| Generate Structs           | 32 seconds | 47.06%     | Unchanged. |
| Generate Web Common Traits | 27 seconds | 39.71%     | Unchanged. |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (88.89% of all time).

| name                                                   | time | percentage | comment    |
|--------------------------------------------------------|------|------------|------------|
| Generating tables schema                               | now  | 0.00%      | Unchanged. |
| Generating types schema                                | now  | 0.00%      | Unchanged. |
| Generating allow tables to appear in same query schema | now  | 88.89%     | Unchanged. |

### Time Report for Generate Structs

The total time spent on all tasks was 32 seconds.
The slowest task was `Generate Table Structs` which took 32 seconds (100.00% of all time).

| name                   | time       | percentage | comment    |
|------------------------|------------|------------|------------|
| Generate Types Structs | now        | 0.00%      | Unchanged. |
| Generate Table Structs | 32 seconds | 100.00%    | Unchanged. |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 27 seconds.
The slowest task was `Generate Table Traits` which took 27 seconds (100.00% of all time).

| name                  | time       | percentage | comment    |
|-----------------------|------------|------------|------------|
| Generate Types Traits | now        | 0.00%      | Unchanged. |
| Generate Table Traits | 27 seconds | 100.00%    | Unchanged. |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 27 seconds.
The slowest task was `Generate Foreign Traits` which took 20 seconds (74.07% of all time).

| name                       | time       | percentage | comment    |
|----------------------------|------------|------------|------------|
| Generate CRUD Traits       | now        | 0.00%      | Unchanged. |
| Generate Deletable Traits  | now        | 3.70%      | Unchanged. |
| Generate Upsertable Traits | now        | 0.00%      | Unchanged. |
| Generate Foreign Traits    | 20 seconds | 74.07%     | Unchanged. |
| Generate Insertable Traits | now        | 0.00%      | Unchanged. |
| Generate Updatable Traits  | now        | 11.11%     | Unchanged. |

![Plot](time_requirements_report.png)
