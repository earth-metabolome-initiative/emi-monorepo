# Time Report for Building Core Structures

The total time spent on all tasks was 23 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 920 ms, 180 µs and 635 ns (39.13% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 920 ms, 180 µs and 635 ns | 39.13%     |         |
| Init DB                                   | 5 seconds, 524 ms, 75 µs and 867 ns  | 21.74%     |         |
| Code Generation                           | 8 seconds, 137 ms, 20 µs and 529 ns  | 34.78%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 523 ms, 952 µs and 971 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 122 µs and 896 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 523 ms, 952 µs and 971 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 610 ms, 35 µs and 699 ns (60.00% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 610 ms, 35 µs and 699 ns | 60.00%     |         |
| Initialize Migrations         | 465 ms, 848 µs and 253 ns           | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 448 ms, 69 µs and 19 ns   | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 866 ms, 406 µs and 123 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 866 ms, 406 µs and 123 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 163 ms, 827 µs and 431 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 417 ms, 835 µs and 465 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 151 ms, 556 µs and 612 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 151 ms, 556 µs and 612 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 91 µs and 984 ns    | NaN%       |         |
| Standard column names and types          | 6 ms, 329 µs and 980 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 724 µs and 462 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 124 µs and 393 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 560 ms, 914 µs and 109 ns (87.50% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 560 ms, 914 µs and 109 ns | 87.50%     |         |
| Procedure Codegen | 576 ms, 106 µs and 420 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 770 ms, 654 µs and 892 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 217 µs and 89 ns                     | 0.00%      |         |
| Creating table extension network | 17 ms, 167 µs and 358 ns             | 0.00%      |         |
| Generating Diesel code           | 25 ms, 855 µs and 862 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 770 ms, 654 µs and 892 ns | 85.71%     |         |
| Generate Web Common Traits       | 747 ms, 18 µs and 908 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 12 ms, 821 µs and 825 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 8 ms, 921 µs and 767 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 112 µs and 270 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 12 ms, 821 µs and 825 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 767 ms, 664 µs and 762 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 990 µs and 130 ns              | 0.00%      |         |
| Generate Table Structs | 6 seconds, 767 ms, 664 µs and 762 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 744 ms, 3 µs and 139 ns (NaN% of all time).

| name                  | time                    | percentage | comment |
|-----------------------|-------------------------|------------|---------|
| Generate Types Traits | 3 ms, 15 µs and 769 ns  | NaN%       |         |
| Generate Table Traits | 744 ms, 3 µs and 139 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 394 ms, 739 µs and 706 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 14 ms, 610 µs and 154 ns  | NaN%       |         |
| Generate Deletable Traits  | 118 ms, 901 µs and 942 ns | NaN%       |         |
| Generate Upsertable Traits | 34 ms, 14 µs and 81 ns    | NaN%       |         |
| Generate Foreign Traits    | 147 ms, 353 µs and 475 ns | NaN%       |         |
| Generate Insertable Traits | 394 ms, 739 µs and 706 ns | NaN%       |         |
| Generate Updatable Traits  | 34 ms, 383 µs and 781 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 541 ms, 132 µs and 105 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 34 ms, 974 µs and 315 ns  | NaN%       |         |
| procedure template Impl Codegen | 541 ms, 132 µs and 105 ns | NaN%       |         |

![Plot](time_requirements_report.png)
