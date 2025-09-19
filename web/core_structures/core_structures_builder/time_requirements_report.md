# Time Report for Building Core Structures

The total time spent on all tasks was 23 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 15 ms, 297 µs and 714 ns (39.13% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 15 ms, 297 µs and 714 ns  | 39.13%     |         |
| Init DB                                   | 6 seconds, 445 ms, 283 µs and 177 ns | 26.09%     |         |
| Code Generation                           | 7 seconds, 987 ms, 841 µs and 790 ns | 30.43%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 6 seconds, 445 ms, 156 µs and 605 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 126 µs and 572 ns                    | 0.00%      |         |
| Init DB Transaction | 6 seconds, 445 ms, 156 µs and 605 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 4 seconds, 334 ms, 130 µs and 376 ns (66.67% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 334 ms, 130 µs and 376 ns | 66.67%     |         |
| Initialize Migrations         | 528 ms, 210 µs and 135 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 582 ms, 816 µs and 94 ns   | 16.67%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 896 ms, 181 µs and 855 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 896 ms, 181 µs and 855 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 175 ms, 600 µs and 566 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 511 ms, 33 µs and 673 ns  | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 160 ms, 956 µs and 692 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 160 ms, 956 µs and 692 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 319 µs and 580 ns   | NaN%       |         |
| Standard column names and types          | 8 ms, 217 µs and 414 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 782 µs and 392 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 324 µs and 488 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 432 ms, 112 µs and 811 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 432 ms, 112 µs and 811 ns | 100.00%    |         |
| Procedure Codegen | 555 ms, 728 µs and 979 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 681 ms, 130 µs and 230 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 327 µs and 396 ns                    | 0.00%      |         |
| Creating table extension network | 17 ms, 93 µs and 367 ns              | 0.00%      |         |
| Generating Diesel code           | 29 ms, 458 µs and 796 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 681 ms, 130 µs and 230 ns | 85.71%     |         |
| Generate Web Common Traits       | 704 ms, 103 µs and 22 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 15 ms, 427 µs and 601 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 783 µs and 518 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 247 µs and 677 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 15 ms, 427 µs and 601 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 677 ms, 964 µs and 362 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 165 µs and 868 ns              | 0.00%      |         |
| Generate Table Structs | 6 seconds, 677 ms, 964 µs and 362 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 700 ms, 968 µs and 301 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 134 µs and 721 ns   | NaN%       |         |
| Generate Table Traits | 700 ms, 968 µs and 301 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 407 ms, 858 µs and 430 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 14 ms, 323 µs and 98 ns   | NaN%       |         |
| Generate Deletable Traits  | 33 ms, 114 µs and 518 ns  | NaN%       |         |
| Generate Upsertable Traits | 44 ms, 678 µs and 633 ns  | NaN%       |         |
| Generate Foreign Traits    | 167 ms, 929 µs and 110 ns | NaN%       |         |
| Generate Insertable Traits | 407 ms, 858 µs and 430 ns | NaN%       |         |
| Generate Updatable Traits  | 33 ms, 64 µs and 512 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 520 ms, 796 µs and 246 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 34 ms, 932 µs and 733 ns  | NaN%       |         |
| procedure template Impl Codegen | 520 ms, 796 µs and 246 ns | NaN%       |         |

![Plot](time_requirements_report.png)
