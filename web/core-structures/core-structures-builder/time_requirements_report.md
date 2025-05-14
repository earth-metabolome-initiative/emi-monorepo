# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took a minute (100.00% of all time).

| name            | time     | percentage | comment |
|-----------------|----------|------------|---------|
| Code generation | a minute | 100.00%    |         |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Structs` which took 41 seconds (51.25% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Retrieving tables          | now        | 0.00%      |         |
| Generating Diesel code     | now        | 11.25%     |         |
| Generate Structs           | 41 seconds | 51.25%     |         |
| Generate Web Common Traits | 29 seconds | 36.25%     |         |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment |
|--------------------------------------------------------|------|------------|---------|
| Generating tables schema                               | now  | 0.00%      |         |
| Generating types schema                                | now  | 0.00%      |         |
| Generating allow tables to appear in same query schema | now  | 100.00%    |         |

### Time Report for Generate Structs

The total time spent on all tasks was 41 seconds.
The slowest task was `Generate Table Structs` which took 41 seconds (100.00% of all time).

| name                   | time       | percentage | comment |
|------------------------|------------|------------|---------|
| Generate Types Structs | now        | 0.00%      |         |
| Generate Table Structs | 41 seconds | 100.00%    |         |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 29 seconds.
The slowest task was `Generate Table Traits` which took 29 seconds (100.00% of all time).

| name                  | time       | percentage | comment |
|-----------------------|------------|------------|---------|
| Generate Types Traits | now        | 0.00%      |         |
| Generate Table Traits | 29 seconds | 100.00%    |         |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 29 seconds.
The slowest task was `Generate Foreign Traits` which took 20 seconds (68.97% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Generate CRUD Traits       | now        | 0.00%      |         |
| Generate Deletable Traits  | now        | 6.90%      |         |
| Generate Upsertable Traits | now        | 0.00%      |         |
| Generate Foreign Traits    | 20 seconds | 68.97%     |         |
| Generate Insertable Traits | now        | 3.45%      |         |
| Generate Updatable Traits  | now        | 13.79%     |         |

![Plot](time_requirements_report.png)
