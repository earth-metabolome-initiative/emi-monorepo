# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 430 ms, 543 µs and 85 ns (38.10% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 430 ms, 543 µs and 85 ns  | 38.10%     |         |
| Init DB                                   | 5 seconds, 719 ms, 957 µs and 938 ns | 23.81%     |         |
| Code Generation                           | 6 seconds, 885 ms, 292 µs and 326 ns | 28.57%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 719 ms, 847 µs and 100 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 110 µs and 838 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 719 ms, 847 µs and 100 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 908 ms, 287 µs and 144 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 908 ms, 287 µs and 144 ns | 60.00%     |         |
| Initialize Migrations         | 476 ms, 353 µs and 773 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 335 ms, 206 µs and 183 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 785 ms, 393 µs and 895 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 785 ms, 393 µs and 895 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 141 ms, 988 µs and 752 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 407 ms, 823 µs and 536 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 129 ms, 577 µs and 287 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 129 ms, 577 µs and 287 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 171 µs and 315 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 70 µs and 277 ns    | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 893 µs and 359 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 276 µs and 514 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 687 ms, 363 µs and 702 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 687 ms, 363 µs and 702 ns | 100.00%    |         |
| Procedure Codegen | 197 ms, 928 µs and 624 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 991 ms, 116 µs and 89 ns (83.33% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 184 µs and 509 ns                   | 0.00%      |         |
| Creating table extension network | 14 ms, 249 µs and 108 ns            | 0.00%      |         |
| Generating Diesel code           | 29 ms, 135 µs and 10 ns             | 0.00%      |         |
| Generate Structs                 | 5 seconds, 991 ms, 116 µs and 89 ns | 83.33%     |         |
| Generate Web Common Traits       | 652 ms, 678 µs and 986 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 651 µs and 738 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 10 ms, 161 µs and 414 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 321 µs and 858 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 651 µs and 738 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 988 ms, 321 µs and 94 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 794 µs and 995 ns             | 0.00%      |         |
| Generate Table Structs | 5 seconds, 988 ms, 321 µs and 94 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 650 ms, 18 µs and 976 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 2 ms, 660 µs and 10 ns   | NaN%       |         |
| Generate Table Traits | 650 ms, 18 µs and 976 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 362 ms, 98 µs and 79 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 184 µs and 366 ns  | NaN%       |         |
| Generate Deletable Traits  | 33 ms, 350 µs and 968 ns  | NaN%       |         |
| Generate Upsertable Traits | 37 ms, 335 µs and 966 ns  | NaN%       |         |
| Generate Foreign Traits    | 148 ms, 351 µs and 491 ns | NaN%       |         |
| Generate Insertable Traits | 362 ms, 98 µs and 79 ns   | NaN%       |         |
| Generate Updatable Traits  | 53 ms, 698 µs and 106 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 185 ms, 594 µs and 767 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 12 ms, 333 µs and 857 ns  | NaN%       |         |
| procedure template Impl Codegen | 185 ms, 594 µs and 767 ns | NaN%       |         |

![Plot](time_requirements_report.png)
