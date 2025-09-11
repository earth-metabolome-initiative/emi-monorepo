# Time Report for Building Core Structures

The total time spent on all tasks was 23 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 10 seconds, 212 ms, 955 µs and 878 ns (43.48% of all time).

| name                                      | time                                  | percentage | comment |
|-------------------------------------------|---------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 10 seconds, 212 ms, 955 µs and 878 ns | 43.48%     |         |
| Init DB                                   | 6 seconds, 85 ms, 914 µs and 493 ns   | 26.09%     |         |
| Code Generation                           | 7 seconds, 91 ms, 268 µs and 204 ns   | 30.43%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 6 seconds, 85 ms, 791 µs and 947 ns (100.00% of all time).

| name                | time                                | percentage | comment |
|---------------------|-------------------------------------|------------|---------|
| Retrieve CSVs       | 122 µs and 546 ns                   | 0.00%      |         |
| Init DB Transaction | 6 seconds, 85 ms, 791 µs and 947 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 4 seconds, 86 ms, 744 µs and 776 ns (66.67% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 86 ms, 744 µs and 776 ns | 66.67%     |         |
| Initialize Migrations         | 497 ms, 578 µs and 678 ns           | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 501 ms, 468 µs and 493 ns | 16.67%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 898 ms, 135 µs and 671 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 898 ms, 135 µs and 671 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 149 ms, 388 µs and 318 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 453 ms, 944 µs and 504 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 136 ms, 486 µs and 70 ns (NaN% of all time).

| name                                     | time                     | percentage | comment |
|------------------------------------------|--------------------------|------------|---------|
| Compatible foreign type constraints      | 136 ms, 486 µs and 70 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 96 µs and 722 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 524 µs and 354 ns  | NaN%       |         |
| Not-null constraints on standard columns | 4 ms, 238 µs and 792 ns  | NaN%       |         |
| Word deprecation constraints             | 1 ms, 42 µs and 380 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 909 ms, 697 µs and 673 ns (85.71% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 909 ms, 697 µs and 673 ns | 85.71%     |         |
| Procedure Codegen | 181 ms, 570 µs and 531 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 208 ms, 255 µs and 355 ns (100.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 187 µs and 845 ns                    | 0.00%      |         |
| Creating table extension network | 18 ms, 621 µs and 600 ns             | 0.00%      |         |
| Generating Diesel code           | 28 ms, 707 µs and 981 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 208 ms, 255 µs and 355 ns | 100.00%    |         |
| Generate Web Common Traits       | 653 ms, 924 µs and 892 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 560 µs and 348 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 10 ms, 151 µs and 128 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 996 µs and 505 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 560 µs and 348 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 205 ms, 489 µs and 24 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 766 µs and 331 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 205 ms, 489 µs and 24 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 651 ms, 288 µs and 348 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 636 µs and 544 ns   | NaN%       |         |
| Generate Table Traits | 651 ms, 288 µs and 348 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 360 ms, 36 µs and 999 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 141 µs and 97 ns   | NaN%       |         |
| Generate Deletable Traits  | 34 ms, 657 µs and 174 ns  | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 156 µs and 335 ns  | NaN%       |         |
| Generate Foreign Traits    | 151 ms, 182 µs and 273 ns | NaN%       |         |
| Generate Insertable Traits | 360 ms, 36 µs and 999 ns  | NaN%       |         |
| Generate Updatable Traits  | 54 ms, 114 µs and 470 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 169 ms, 583 µs and 842 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 986 µs and 689 ns  | NaN%       |         |
| procedure template Impl Codegen | 169 ms, 583 µs and 842 ns | NaN%       |         |

![Plot](time_requirements_report.png)
