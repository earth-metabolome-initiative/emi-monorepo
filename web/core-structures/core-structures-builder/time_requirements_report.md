# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took a minute (100.00% of all time).

| name            | time     | percentage | comment |
|-----------------|----------|------------|---------|
| Code generation | a minute | 100.00%    |         |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Structs` which took 31 seconds (49.21% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Retrieving tables          | now        | 0.00%      |         |
| Generating Diesel code     | now        | 11.11%     |         |
| Generate Structs           | 31 seconds | 49.21%     |         |
| Generate Web Common Traits | 25 seconds | 39.68%     |         |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment |
|--------------------------------------------------------|------|------------|---------|
| Generating tables schema                               | now  | 0.00%      |         |
| Generating types schema                                | now  | 0.00%      |         |
| Generating allow tables to appear in same query schema | now  | 100.00%    |         |

### Time Report for Generate Structs

The total time spent on all tasks was 31 seconds.
The slowest task was `Generate Table Structs` which took 31 seconds (100.00% of all time).

| name                   | time       | percentage | comment |
|------------------------|------------|------------|---------|
| Generate Types Structs | now        | 0.00%      |         |
| Generate Table Structs | 31 seconds | 100.00%    |         |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 25 seconds.
The slowest task was `Generate Table Traits` which took 25 seconds (100.00% of all time).

| name                  | time       | percentage | comment |
|-----------------------|------------|------------|---------|
| Generate Types Traits | now        | 0.00%      |         |
| Generate Table Traits | 25 seconds | 100.00%    |         |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 25 seconds.
The slowest task was `Generate Foreign Traits` which took 19 seconds (76.00% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Generate CRUD Traits       | now        | 0.00%      |         |
| Generate Deletable Traits  | now        | 4.00%      |         |
| Generate Upsertable Traits | now        | 0.00%      |         |
| Generate Foreign Traits    | 19 seconds | 76.00%     |         |
| Generate Insertable Traits | now        | 0.00%      |         |
| Generate Updatable Traits  | now        | 12.00%     |         |

![Plot](time_requirements_report.png)
