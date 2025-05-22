# Time Report for Building Core Structures

The total time spent on all tasks was a minute.
The slowest task was `Code generation` which took a minute (100.00% of all time).

| name            | time     | percentage | comment |
|-----------------|----------|------------|---------|
| Code generation | a minute | 100.00%    |         |

## Time Report for Code generation

The total time spent on all tasks was a minute.
The slowest task was `Generate Structs` which took a minute (52.27% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Retrieving tables          | now        | 0.00%      |         |
| Generating Diesel code     | now        | 10.23%     |         |
| Generate Structs           | a minute   | 52.27%     |         |
| Generate Web Common Traits | 32 seconds | 36.36%     |         |

### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (100.00% of all time).

| name                                                   | time | percentage | comment |
|--------------------------------------------------------|------|------------|---------|
| Generating tables schema                               | now  | 0.00%      |         |
| Generating types schema                                | now  | 0.00%      |         |
| Generating allow tables to appear in same query schema | now  | 100.00%    |         |

### Time Report for Generate Structs

The total time spent on all tasks was a minute.
The slowest task was `Generate Table Structs` which took a minute (100.00% of all time).

| name                   | time     | percentage | comment |
|------------------------|----------|------------|---------|
| Generate Types Structs | now      | 0.00%      |         |
| Generate Table Structs | a minute | 100.00%    |         |

### Time Report for Generate Web Common Traits

The total time spent on all tasks was 32 seconds.
The slowest task was `Generate Table Traits` which took 32 seconds (100.00% of all time).

| name                  | time       | percentage | comment |
|-----------------------|------------|------------|---------|
| Generate Types Traits | now        | 0.00%      |         |
| Generate Table Traits | 32 seconds | 100.00%    |         |

#### Time Report for Generate Table Traits

The total time spent on all tasks was 32 seconds.
The slowest task was `Generate Foreign Traits` which took 21 seconds (65.62% of all time).

| name                       | time       | percentage | comment |
|----------------------------|------------|------------|---------|
| Generate CRUD Traits       | now        | 0.00%      |         |
| Generate Deletable Traits  | now        | 6.25%      |         |
| Generate Upsertable Traits | now        | 0.00%      |         |
| Generate Foreign Traits    | 21 seconds | 65.62%     |         |
| Generate Insertable Traits | now        | 3.12%      |         |
| Generate Updatable Traits  | now        | 15.62%     |         |

![Plot](time_requirements_report.png)
