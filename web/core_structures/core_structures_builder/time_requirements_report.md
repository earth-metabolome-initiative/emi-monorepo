# Time Report for Building Core Structures

The total time spent on all tasks was 20 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 234 ms, 613 µs and 424 ns (40.00% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 234 ms, 613 µs and 424 ns | 40.00%     |         |
| Init DB                                   | 5 seconds, 910 ms, 672 µs and 214 ns | 25.00%     |         |
| Code Generation                           | 6 seconds, 842 ms, 726 µs and 942 ns | 30.00%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 910 ms, 561 µs and 687 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 110 µs and 527 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 910 ms, 561 µs and 687 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 4 seconds, 38 ms, 176 µs and 939 ns (80.00% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 38 ms, 176 µs and 939 ns | 80.00%     |         |
| Initialize Migrations         | 502 ms, 95 µs and 406 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 370 ms, 289 µs and 342 ns | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 819 ms, 544 µs and 102 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 819 ms, 544 µs and 102 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 149 ms, 156 µs and 573 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 401 ms, 588 µs and 667 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 137 ms, 286 µs and 232 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 137 ms, 286 µs and 232 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 122 µs and 389 ns   | NaN%       |         |
| Standard column names and types          | 5 ms, 829 µs and 725 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 943 µs and 180 ns   | NaN%       |         |
| Word deprecation constraints             | 975 µs and 47 ns          | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 653 ms, 67 µs and 770 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 653 ms, 67 µs and 770 ns | 100.00%    |         |
| Procedure Codegen | 189 ms, 659 µs and 172 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 990 ms, 687 µs and 886 ns (83.33% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 189 µs and 357 ns                    | 0.00%      |         |
| Creating table extension network | 16 ms, 439 µs and 779 ns             | 0.00%      |         |
| Generating Diesel code           | 28 ms, 296 µs and 227 ns             | 0.00%      |         |
| Generate Structs                 | 5 seconds, 990 ms, 687 µs and 886 ns | 83.33%     |         |
| Generate Web Common Traits       | 617 ms, 454 µs and 521 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 515 µs and 64 ns (NaN% of all time).

| name                                                   | time                    | percentage | comment |
|--------------------------------------------------------|-------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 753 µs and 265 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 27 µs and 898 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 515 µs and 64 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 988 ms, 46 µs and 307 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 641 µs and 579 ns             | 0.00%      |         |
| Generate Table Structs | 5 seconds, 988 ms, 46 µs and 307 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 614 ms, 772 µs and 841 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 681 µs and 680 ns   | NaN%       |         |
| Generate Table Traits | 614 ms, 772 µs and 841 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 338 ms, 766 µs and 813 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 14 ms, 921 µs and 290 ns  | NaN%       |         |
| Generate Deletable Traits  | 33 ms, 105 µs and 290 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 543 µs and 901 ns  | NaN%       |         |
| Generate Foreign Traits    | 143 ms, 288 µs and 621 ns | NaN%       |         |
| Generate Insertable Traits | 338 ms, 766 µs and 813 ns | NaN%       |         |
| Generate Updatable Traits  | 49 ms, 146 µs and 926 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 178 ms, 271 µs and 432 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 387 µs and 740 ns  | NaN%       |         |
| procedure template Impl Codegen | 178 ms, 271 µs and 432 ns | NaN%       |         |

![Plot](time_requirements_report.png)
