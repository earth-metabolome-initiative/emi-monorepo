# Time Report for Building Core Structures

The total time spent on all tasks was 24 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 10 seconds, 213 ms, 678 µs and 125 ns (41.67% of all time).

| name                                      | time                                  | percentage | comment |
|-------------------------------------------|---------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 10 seconds, 213 ms, 678 µs and 125 ns | 41.67%     |         |
| Init DB                                   | 5 seconds, 906 ms, 968 µs and 599 ns  | 20.83%     |         |
| Code Generation                           | 7 seconds, 967 ms, 740 µs and 76 ns   | 29.17%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 906 ms, 827 µs and 836 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 140 µs and 763 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 906 ms, 827 µs and 836 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 4 seconds, 32 ms, 692 µs and 749 ns (80.00% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 32 ms, 692 µs and 749 ns | 80.00%     |         |
| Initialize Migrations         | 485 ms, 733 µs and 270 ns           | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 388 ms, 401 µs and 817 ns | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 808 ms, 622 µs and 756 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 808 ms, 622 µs and 756 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 147 ms, 143 µs and 166 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 432 ms, 635 µs and 895 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 134 ms, 399 µs and 579 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 134 ms, 399 µs and 579 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 295 µs and 923 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 397 µs and 733 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 771 µs and 94 ns    | NaN%       |         |
| Word deprecation constraints             | 1 ms, 278 µs and 837 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 392 ms, 105 µs and 914 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 392 ms, 105 µs and 914 ns | 100.00%    |         |
| Procedure Codegen | 575 ms, 634 µs and 162 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 748 ms, 863 µs and 195 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 322 µs and 539 ns                    | 0.00%      |         |
| Creating table extension network | 21 ms, 286 µs and 49 ns              | 0.00%      |         |
| Generating Diesel code           | 27 ms, 999 µs and 69 ns              | 0.00%      |         |
| Generate Structs                 | 6 seconds, 748 ms, 863 µs and 195 ns | 85.71%     |         |
| Generate Web Common Traits       | 593 ms, 635 µs and 62 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 805 µs and 821 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 147 µs and 398 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 45 µs and 850 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 805 µs and 821 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 745 ms, 829 µs and 148 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 34 µs and 47 ns                | 0.00%      |         |
| Generate Table Structs | 6 seconds, 745 ms, 829 µs and 148 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 591 ms, 126 µs and 201 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 508 µs and 861 ns   | NaN%       |         |
| Generate Table Traits | 591 ms, 126 µs and 201 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 345 ms, 317 µs and 345 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 587 µs and 896 ns  | NaN%       |         |
| Generate Deletable Traits  | 28 ms, 280 µs and 175 ns  | NaN%       |         |
| Generate Upsertable Traits | 33 ms, 217 µs and 74 ns   | NaN%       |         |
| Generate Foreign Traits    | 135 ms, 338 µs and 852 ns | NaN%       |         |
| Generate Insertable Traits | 345 ms, 317 µs and 345 ns | NaN%       |         |
| Generate Updatable Traits  | 35 ms, 384 µs and 859 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 537 ms, 749 µs and 896 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 37 ms, 884 µs and 266 ns  | NaN%       |         |
| procedure template Impl Codegen | 537 ms, 749 µs and 896 ns | NaN%       |         |

![Plot](time_requirements_report.png)
