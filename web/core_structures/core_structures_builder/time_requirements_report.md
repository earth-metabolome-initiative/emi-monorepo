# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 30 ms, 904 µs and 786 ns (42.86% of all time).

| name                                      | time                                | percentage | comment |
|-------------------------------------------|-------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 30 ms, 904 µs and 786 ns | 42.86%     |         |
| Init DB                                   | 5 seconds, 796 ms, 67 µs and 107 ns | 23.81%     |         |
| Code Generation                           | 7 seconds, 49 ms, 167 µs and 524 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 795 ms, 944 µs and 771 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 122 µs and 336 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 795 ms, 944 µs and 771 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 928 ms, 797 µs and 138 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 928 ms, 797 µs and 138 ns | 60.00%     |         |
| Initialize Migrations         | 504 ms, 408 µs and 129 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 362 ms, 739 µs and 504 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 821 ms, 167 µs and 807 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 821 ms, 167 µs and 807 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 146 ms, 47 µs and 857 ns  | 0.00%      |         |
| Procedure and procedure template check constraints | 395 ms, 523 µs and 840 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 134 ms, 342 µs and 413 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 134 ms, 342 µs and 413 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 142 µs and 871 ns   | NaN%       |         |
| Standard column names and types          | 5 ms, 883 µs and 464 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 701 µs and 408 ns   | NaN%       |         |
| Word deprecation constraints             | 977 µs and 701 ns         | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 811 ms, 620 µs and 667 ns (85.71% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 811 ms, 620 µs and 667 ns | 85.71%     |         |
| Procedure Codegen | 237 ms, 546 µs and 857 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 92 ms, 205 µs and 132 ns (100.00% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 281 µs and 377 ns                   | 0.00%      |         |
| Creating table extension network | 13 ms, 573 µs and 547 ns            | 0.00%      |         |
| Generating Diesel code           | 28 ms, 764 µs and 42 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 92 ms, 205 µs and 132 ns | 100.00%    |         |
| Generate Web Common Traits       | 676 ms, 796 µs and 569 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 472 µs and 969 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 10 ms, 44 µs and 980 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 246 µs and 93 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 472 µs and 969 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 89 ms, 254 µs and 862 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 950 µs and 270 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 89 ms, 254 µs and 862 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 673 ms, 598 µs and 754 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 197 µs and 815 ns   | NaN%       |         |
| Generate Table Traits | 673 ms, 598 µs and 754 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 373 ms, 362 µs and 131 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 17 ms, 634 µs and 300 ns  | NaN%       |         |
| Generate Deletable Traits  | 37 ms, 681 µs and 644 ns  | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 501 µs and 86 ns   | NaN%       |         |
| Generate Foreign Traits    | 151 ms, 773 µs and 834 ns | NaN%       |         |
| Generate Insertable Traits | 373 ms, 362 µs and 131 ns | NaN%       |         |
| Generate Updatable Traits  | 56 ms, 645 µs and 759 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 219 ms, 576 µs and 528 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 17 ms, 970 µs and 329 ns  | NaN%       |         |
| procedure template Impl Codegen | 219 ms, 576 µs and 528 ns | NaN%       |         |

![Plot](time_requirements_report.png)
