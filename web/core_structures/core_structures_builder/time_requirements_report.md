# Time Report for Building Core Structures

The total time spent on all tasks was 2 minutes.
The slowest task was `Init DB Transaction` which took 2 minutes (95.80% of all time).

| name                | time      | percentage | comment |
|---------------------|-----------|------------|---------|
| Init DB Transaction | 2 minutes | 95.80%     |         |
| Code Generation     | now       | 4.20%      |         |

## Time Report for Init DB Transaction

The total time spent on all tasks was 2 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes (96.49% of all time).

| name                          | time      | percentage | comment |
|-------------------------------|-----------|------------|---------|
| Initialize CSVs               | now       | 2.63%      |         |
| Initialize Migrations         | now       | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes | 96.49%     |         |

### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Check constraints in schema 'procedures'` which took a minute (49.09% of all time).

| name                                               | time     | percentage | comment |
|----------------------------------------------------|----------|------------|---------|
| Check constraints in schema 'public'               | a minute | 44.55%     |         |
| Check constraints in schema 'procedures'           | a minute | 49.09%     |         |
| Check constraints in schema 'procedure_templates'  | now      | 3.64%      |         |
| Procedure and procedure template check constraints | now      | 0.00%      |         |
| Procedure and procedure template asset constraints | now      | 0.91%      |         |

#### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was a minute.
The slowest task was `Compatible foreign type constraints` which took a minute (97.96% of all time).

| name                                     | time     | percentage | comment |
|------------------------------------------|----------|------------|---------|
| Compatible foreign type constraints      | a minute | 97.96%     |         |
| Lowercase column and table names         | now      | 0.00%      |         |
| Standard column names and types          | now      | 0.00%      |         |
| Not-null constraints on standard columns | now      | 0.00%      |         |
| Same-as constraint no cascade            | now      | 0.00%      |         |
| Word deprecation constraints             | now      | 0.00%      |         |

#### Time Report for Check constraints in schema 'procedures'

The total time spent on all tasks was a minute.
The slowest task was `Compatible foreign type constraints` which took a minute (100.00% of all time).

| name                                     | time     | percentage | comment |
|------------------------------------------|----------|------------|---------|
| Compatible foreign type constraints      | a minute | 100.00%    |         |
| Lowercase column and table names         | now      | 0.00%      |         |
| Standard column names and types          | now      | 0.00%      |         |
| Not-null constraints on standard columns | now      | 0.00%      |         |
| Same-as constraint no cascade            | now      | 0.00%      |         |
| Word deprecation constraints             | now      | 0.00%      |         |

#### Time Report for Check constraints in schema 'procedure_templates'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took now (100.00% of all time).

| name                                     | time | percentage | comment |
|------------------------------------------|------|------------|---------|
| Compatible foreign type constraints      | now  | 100.00%    |         |
| Lowercase column and table names         | now  | 0.00%      |         |
| Standard column names and types          | now  | 0.00%      |         |
| Not-null constraints on standard columns | now  | 0.00%      |         |
| Same-as constraint no cascade            | now  | 0.00%      |         |
| Word deprecation constraints             | now  | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took now (100.00% of all time).

| name              | time | percentage | comment |
|-------------------|------|------------|---------|
| Code generation   | now  | 100.00%    |         |
| Procedure Codegen | now  | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took now (60.00% of all time).

| name                             | time | percentage | comment |
|----------------------------------|------|------------|---------|
| Retrieving tables                | now  | 0.00%      |         |
| Creating column same-as network  | now  | 20.00%     |         |
| Creating table extension network | now  | 0.00%      |         |
| Generating Diesel code           | now  | 0.00%      |         |
| Generate Structs                 | now  | 60.00%     |         |
| Generate Web Common Traits       | now  | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took now (NaN% of all time).

| name                                                   | time | percentage | comment |
|--------------------------------------------------------|------|------------|---------|
| Generating tables schema                               | now  | NaN%       |         |
| Generating types schema                                | now  | NaN%       |         |
| Generating allow tables to appear in same query schema | now  | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took now (100.00% of all time).

| name                   | time | percentage | comment |
|------------------------|------|------------|---------|
| Generate Types Structs | now  | 0.00%      |         |
| Generate Table Structs | now  | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took now (NaN% of all time).

| name                  | time | percentage | comment |
|-----------------------|------|------------|---------|
| Generate Types Traits | now  | NaN%       |         |
| Generate Table Traits | now  | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Foreign Traits` which took now (NaN% of all time).

| name                       | time | percentage | comment |
|----------------------------|------|------------|---------|
| Generate CRUD Traits       | now  | NaN%       |         |
| Generate Deletable Traits  | now  | NaN%       |         |
| Generate Upsertable Traits | now  | NaN%       |         |
| Generate Foreign Traits    | now  | NaN%       |         |
| Generate Insertable Traits | now  | NaN%       |         |
| Generate Updatable Traits  | now  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took now (NaN% of all time).

| name                            | time | percentage | comment |
|---------------------------------|------|------------|---------|
| Procedure Impl Codegen          | now  | NaN%       |         |
| procedure template Impl Codegen | now  | NaN%       |         |

![Plot](time_requirements_report.png)
