# Time Report for Building Core Structures

The total time spent on all tasks was 23 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 10 seconds, 108 ms, 585 µs and 28 ns (43.48% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 10 seconds, 108 ms, 585 µs and 28 ns | 43.48%     |         |
| Init DB                                   | 5 seconds, 414 ms, 913 µs and 272 ns | 21.74%     |         |
| Code Generation                           | 7 seconds, 495 ms, 387 µs and 200 ns | 30.43%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 414 ms, 786 µs and 921 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 126 µs and 351 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 414 ms, 786 µs and 921 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 611 ms, 87 µs and 351 ns (60.00% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 611 ms, 87 µs and 351 ns | 60.00%     |         |
| Initialize Migrations         | 459 ms, 88 µs and 665 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 344 ms, 610 µs and 905 ns | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 780 ms, 779 µs and 728 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 780 ms, 779 µs and 728 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 166 ms, 726 µs and 300 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 397 ms, 104 µs and 877 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 154 ms, 482 µs and 222 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 154 ms, 482 µs and 222 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 14 µs and 137 ns    | NaN%       |         |
| Standard column names and types          | 6 ms, 326 µs and 864 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 900 µs and 158 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 2 µs and 919 ns     | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 964 ms, 16 µs and 536 ns (85.71% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 964 ms, 16 µs and 536 ns | 85.71%     |         |
| Procedure Codegen | 531 ms, 370 µs and 664 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 319 ms, 860 µs and 135 ns (100.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 230 µs and 8 ns                      | 0.00%      |         |
| Creating table extension network | 25 ms, 156 µs and 819 ns             | 0.00%      |         |
| Generating Diesel code           | 27 ms, 687 µs and 173 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 319 ms, 860 µs and 135 ns | 100.00%    |         |
| Generate Web Common Traits       | 591 ms, 82 µs and 401 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 13 ms, 626 µs and 302 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 359 µs and 889 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 700 µs and 982 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 13 ms, 626 µs and 302 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 317 ms, 22 µs and 216 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 837 µs and 919 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 317 ms, 22 µs and 216 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 588 ms, 285 µs and 894 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 796 µs and 507 ns   | NaN%       |         |
| Generate Table Traits | 588 ms, 285 µs and 894 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 343 ms, 646 µs and 784 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 362 µs and 152 ns  | NaN%       |         |
| Generate Deletable Traits  | 29 ms, 406 µs and 728 ns  | NaN%       |         |
| Generate Upsertable Traits | 34 ms, 960 µs and 712 ns  | NaN%       |         |
| Generate Foreign Traits    | 135 ms, 838 µs and 910 ns | NaN%       |         |
| Generate Insertable Traits | 343 ms, 646 µs and 784 ns | NaN%       |         |
| Generate Updatable Traits  | 31 ms, 70 µs and 608 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 499 ms, 843 µs and 894 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 31 ms, 526 µs and 770 ns  | NaN%       |         |
| procedure template Impl Codegen | 499 ms, 843 µs and 894 ns | NaN%       |         |

![Plot](time_requirements_report.png)
