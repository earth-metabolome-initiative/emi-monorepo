# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 834 ms, 433 µs and 535 ns (38.10% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 834 ms, 433 µs and 535 ns | 38.10%     |         |
| Init DB                                   | 5 seconds, 872 ms, 182 µs and 196 ns | 23.81%     |         |
| Code Generation                           | 6 seconds, 923 ms, 952 µs and 236 ns | 28.57%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 872 ms, 73 µs and 901 ns (100.00% of all time).

| name                | time                                | percentage | comment |
|---------------------|-------------------------------------|------------|---------|
| Retrieve CSVs       | 108 µs and 295 ns                   | 0.00%      |         |
| Init DB Transaction | 5 seconds, 872 ms, 73 µs and 901 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 968 ms, 198 µs and 746 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 968 ms, 198 µs and 746 ns | 60.00%     |         |
| Initialize Migrations         | 500 ms, 51 µs and 510 ns             | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 403 ms, 823 µs and 645 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 823 ms, 541 µs and 714 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 823 ms, 541 µs and 714 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 153 ms, 17 µs and 783 ns  | 0.00%      |         |
| Procedure and procedure template check constraints | 427 ms, 264 µs and 148 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 141 ms, 7 µs and 696 ns (NaN% of all time).

| name                                     | time                    | percentage | comment |
|------------------------------------------|-------------------------|------------|---------|
| Compatible foreign type constraints      | 141 ms, 7 µs and 696 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 142 µs and 11 ns  | NaN%       |         |
| Standard column names and types          | 5 ms, 781 µs and 24 ns  | NaN%       |         |
| Not-null constraints on standard columns | 4 ms, 84 µs and 352 ns  | NaN%       |         |
| Word deprecation constraints             | 1 ms, 2 µs and 700 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 737 ms, 47 µs and 395 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 737 ms, 47 µs and 395 ns | 100.00%    |         |
| Procedure Codegen | 186 ms, 904 µs and 841 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 46 ms, 23 µs and 717 ns (100.00% of all time).

| name                             | time                               | percentage | comment |
|----------------------------------|------------------------------------|------------|---------|
| Retrieving tables                | 1 ms, 378 µs and 560 ns            | 0.00%      |         |
| Creating table extension network | 16 ms, 137 µs and 893 ns           | 0.00%      |         |
| Generating Diesel code           | 29 ms, 934 µs and 909 ns           | 0.00%      |         |
| Generate Structs                 | 6 seconds, 46 ms, 23 µs and 717 ns | 100.00%    |         |
| Generate Web Common Traits       | 643 ms, 572 µs and 316 ns          | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 15 ms, 674 µs and 591 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 936 µs and 723 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 323 µs and 595 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 15 ms, 674 µs and 591 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 42 ms, 521 µs and 728 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 501 µs and 989 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 42 ms, 521 µs and 728 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 641 ms, 302 µs and 284 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 270 µs and 32 ns    | NaN%       |         |
| Generate Table Traits | 641 ms, 302 µs and 284 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 355 ms, 484 µs and 411 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 43 µs and 735 ns   | NaN%       |         |
| Generate Deletable Traits  | 35 ms, 58 µs and 616 ns   | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 293 µs and 637 ns  | NaN%       |         |
| Generate Foreign Traits    | 149 ms, 106 µs and 572 ns | NaN%       |         |
| Generate Insertable Traits | 355 ms, 484 µs and 411 ns | NaN%       |         |
| Generate Updatable Traits  | 50 ms, 315 µs and 313 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 175 ms, 200 µs and 178 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 704 µs and 663 ns  | NaN%       |         |
| procedure template Impl Codegen | 175 ms, 200 µs and 178 ns | NaN%       |         |

![Plot](time_requirements_report.png)
