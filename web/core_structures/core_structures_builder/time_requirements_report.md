# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took a minute (100.00% of all time).

| name            | time     | percentage | comment |
|-----------------|----------|------------|---------|
| Code generation | a minute | 100.00%    |         |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Structs` which took 45 seconds (53.57% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Retrieving tables          | now        | 0.00%      |         |
| Generating Diesel code     | now        | 11.90%     |         |
| Generate Structs           | 45 seconds | 53.57%     |         |
| Generate Web Common Traits | 29 seconds | 34.52%     |         |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (90.00% of all time).

| name                                                   | time | percentage | comment |
|--------------------------------------------------------|------|------------|---------|
| Generating tables schema                               | now  | 0.00%      |         |
| Generating types schema                                | now  | 0.00%      |         |
| Generating allow tables to appear in same query schema | now  | 90.00%     |         |

### Time Report for Generate Structs

The total time spent on all tasks was 45 seconds.
The slowest task was `Generate Table Structs` which took 44 seconds (97.78% of all time).

| name                   | time       | percentage | comment |
|------------------------|------------|------------|---------|
| Generate Types Structs | now        | 0.00%      |         |
| Generate Table Structs | 44 seconds | 97.78%     |         |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 29 seconds.
The slowest task was `Generate Table Traits` which took 28 seconds (96.55% of all time).

| name                  | time       | percentage | comment |
|-----------------------|------------|------------|---------|
| Generate Types Traits | now        | 0.00%      |         |
| Generate Table Traits | 28 seconds | 96.55%     |         |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 28 seconds.
The slowest task was `Generate Foreign Traits` which took 20 seconds (71.43% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Generate CRUD Traits       | now        | 0.00%      |         |
| Generate Deletable Traits  | now        | 7.14%      |         |
| Generate Upsertable Traits | now        | 0.00%      |         |
| Generate Foreign Traits    | 20 seconds | 71.43%     |         |
| Generate Insertable Traits | now        | 3.57%      |         |
| Generate Updatable Traits  | now        | 14.29%     |         |

![Plot](time_requirements_report.png)
