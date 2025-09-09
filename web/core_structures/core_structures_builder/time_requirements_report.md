# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 769 ms, 191 µs and 9 ns (38.10% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 769 ms, 191 µs and 9 ns   | 38.10%     |         |
| Init DB                                   | 5 seconds, 691 ms, 76 µs and 591 ns  | 23.81%     |         |
| Code Generation                           | 6 seconds, 768 ms, 867 µs and 335 ns | 28.57%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 690 ms, 959 µs and 664 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 116 µs and 927 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 690 ms, 959 µs and 664 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 918 ms, 841 µs and 243 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 918 ms, 841 µs and 243 ns | 60.00%     |         |
| Initialize Migrations         | 469 ms, 142 µs and 560 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 302 ms, 975 µs and 861 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 751 ms, 902 µs and 448 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 751 ms, 902 µs and 448 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 153 ms, 85 µs and 974 ns  | 0.00%      |         |
| Procedure and procedure template check constraints | 397 ms, 987 µs and 439 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 140 ms, 801 µs and 127 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 140 ms, 801 µs and 127 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 137 µs and 653 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 90 µs and 595 ns    | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 780 µs and 686 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 275 µs and 913 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 598 ms, 873 µs and 848 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 598 ms, 873 µs and 848 ns | 100.00%    |         |
| Procedure Codegen | 169 ms, 993 µs and 487 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 928 ms, 393 µs and 702 ns (83.33% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 175 µs and 576 ns                    | 0.00%      |         |
| Creating table extension network | 12 ms, 693 µs and 577 ns             | 0.00%      |         |
| Generating Diesel code           | 28 ms, 308 µs and 460 ns             | 0.00%      |         |
| Generate Structs                 | 5 seconds, 928 ms, 393 µs and 702 ns | 83.33%     |         |
| Generate Web Common Traits       | 629 ms, 302 µs and 533 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 398 µs and 540 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 935 µs and 891 ns  | NaN%       |         |
| Generating types schema                                | 3 ms, 974 µs and 29 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 398 µs and 540 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 925 ms, 324 µs and 764 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 68 µs and 938 ns               | 0.00%      |         |
| Generate Table Structs | 5 seconds, 925 ms, 324 µs and 764 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 626 ms, 191 µs and 731 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 110 µs and 802 ns   | NaN%       |         |
| Generate Table Traits | 626 ms, 191 µs and 731 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 344 ms, 476 µs and 905 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 16 ms, 776 µs and 822 ns  | NaN%       |         |
| Generate Deletable Traits  | 37 ms, 343 µs and 897 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 166 µs and 58 ns   | NaN%       |         |
| Generate Foreign Traits    | 139 ms, 810 µs and 357 ns | NaN%       |         |
| Generate Insertable Traits | 344 ms, 476 µs and 905 ns | NaN%       |         |
| Generate Updatable Traits  | 52 ms, 617 µs and 692 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 158 ms, 501 µs and 309 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 492 µs and 178 ns  | NaN%       |         |
| procedure template Impl Codegen | 158 ms, 501 µs and 309 ns | NaN%       |         |

![Plot](time_requirements_report.png)
