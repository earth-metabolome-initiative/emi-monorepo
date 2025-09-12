# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 788 ms, 18 µs and 386 ns (38.10% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 788 ms, 18 µs and 386 ns  | 38.10%     |         |
| Init DB                                   | 5 seconds, 452 ms, 249 µs and 492 ns | 23.81%     |         |
| Code Generation                           | 7 seconds, 537 ms, 176 µs and 471 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 452 ms, 86 µs and 575 ns (100.00% of all time).

| name                | time                                | percentage | comment |
|---------------------|-------------------------------------|------------|---------|
| Retrieve CSVs       | 162 µs and 917 ns                   | 0.00%      |         |
| Init DB Transaction | 5 seconds, 452 ms, 86 µs and 575 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 641 ms, 614 µs and 138 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 641 ms, 614 µs and 138 ns | 60.00%     |         |
| Initialize Migrations         | 451 ms, 51 µs and 937 ns             | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 359 ms, 420 µs and 500 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 780 ms, 536 µs and 72 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 780 ms, 536 µs and 72 ns  | 0.00%      |         |
| Check constraints in schema 'public'               | 160 ms, 49 µs and 736 ns  | 0.00%      |         |
| Procedure and procedure template check constraints | 418 ms, 834 µs and 692 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 148 ms, 83 µs and 663 ns (NaN% of all time).

| name                                     | time                     | percentage | comment |
|------------------------------------------|--------------------------|------------|---------|
| Compatible foreign type constraints      | 148 ms, 83 µs and 663 ns | NaN%       |         |
| Lowercase column and table names         | 985 µs and 152 ns        | NaN%       |         |
| Standard column names and types          | 6 ms, 211 µs and 908 ns  | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 803 µs and 240 ns  | NaN%       |         |
| Word deprecation constraints             | 965 µs and 773 ns        | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 353 ms, 24 µs and 150 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 7 seconds, 353 ms, 24 µs and 150 ns | 100.00%    |         |
| Procedure Codegen | 184 ms, 152 µs and 321 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 685 ms, 452 µs and 718 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 115 µs and 295 ns                    | 0.00%      |         |
| Creating table extension network | 16 ms, 215 µs and 690 ns             | 0.00%      |         |
| Generating Diesel code           | 26 ms, 225 µs and 439 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 685 ms, 452 µs and 718 ns | 85.71%     |         |
| Generate Web Common Traits       | 625 ms, 15 µs and 8 ns               | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 13 ms, 181 µs and 183 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 87 µs and 653 ns   | NaN%       |         |
| Generating types schema                                | 3 ms, 956 µs and 603 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 13 ms, 181 µs and 183 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 682 ms, 287 µs and 725 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 164 µs and 993 ns              | 0.00%      |         |
| Generate Table Structs | 6 seconds, 682 ms, 287 µs and 725 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 621 ms, 133 µs and 38 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 3 ms, 881 µs and 970 ns  | NaN%       |         |
| Generate Table Traits | 621 ms, 133 µs and 38 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 343 ms, 990 µs and 386 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 266 µs and 322 ns  | NaN%       |         |
| Generate Deletable Traits  | 34 ms, 773 µs and 928 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 362 µs and 79 ns   | NaN%       |         |
| Generate Foreign Traits    | 141 ms, 416 µs and 464 ns | NaN%       |         |
| Generate Insertable Traits | 343 ms, 990 µs and 386 ns | NaN%       |         |
| Generate Updatable Traits  | 50 ms, 323 µs and 859 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 172 ms, 471 µs and 259 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 681 µs and 62 ns   | NaN%       |         |
| procedure template Impl Codegen | 172 ms, 471 µs and 259 ns | NaN%       |         |

![Plot](time_requirements_report.png)
