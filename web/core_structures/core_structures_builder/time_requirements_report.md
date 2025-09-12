# Time Report for Building Core Structures

The total time spent on all tasks was 22 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 609 ms, 604 µs and 96 ns (40.91% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 609 ms, 604 µs and 96 ns  | 40.91%     |         |
| Init DB                                   | 5 seconds, 526 ms, 793 µs and 685 ns | 22.73%     |         |
| Code Generation                           | 7 seconds, 702 ms, 366 µs and 991 ns | 31.82%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 526 ms, 636 µs and 927 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 156 µs and 758 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 526 ms, 636 µs and 927 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 680 ms, 584 µs and 827 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 680 ms, 584 µs and 827 ns | 60.00%     |         |
| Initialize Migrations         | 462 ms, 20 µs and 170 ns             | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 384 ms, 31 µs and 930 ns   | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 813 ms, 780 µs and 846 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 813 ms, 780 µs and 846 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 170 ms, 541 µs and 228 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 399 ms, 709 µs and 856 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 159 ms, 752 µs and 364 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 159 ms, 752 µs and 364 ns | NaN%       |         |
| Lowercase column and table names         | 973 µs and 325 ns         | NaN%       |         |
| Standard column names and types          | 5 ms, 194 µs and 258 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 638 µs and 512 ns   | NaN%       |         |
| Word deprecation constraints             | 982 µs and 769 ns         | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 514 ms, 964 µs and 674 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 514 ms, 964 µs and 674 ns | 100.00%    |         |
| Procedure Codegen | 187 ms, 402 µs and 317 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 835 ms, 468 µs and 63 ns (85.71% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 267 µs and 655 ns                   | 0.00%      |         |
| Creating table extension network | 15 ms, 847 µs and 705 ns            | 0.00%      |         |
| Generating Diesel code           | 24 ms, 949 µs and 993 ns            | 0.00%      |         |
| Generate Structs                 | 6 seconds, 835 ms, 468 µs and 63 ns | 85.71%     |         |
| Generate Web Common Traits       | 638 ms, 431 µs and 258 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 12 ms, 487 µs and 936 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 8 ms, 508 µs and 318 ns  | NaN%       |         |
| Generating types schema                                | 3 ms, 953 µs and 739 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 12 ms, 487 µs and 936 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 832 ms, 604 µs and 886 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 863 µs and 177 ns              | 0.00%      |         |
| Generate Table Structs | 6 seconds, 832 ms, 604 µs and 886 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 635 ms, 935 µs and 828 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 495 µs and 430 ns   | NaN%       |         |
| Generate Table Traits | 635 ms, 935 µs and 828 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 357 ms, 676 µs and 858 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 969 µs and 410 ns  | NaN%       |         |
| Generate Deletable Traits  | 34 ms, 263 µs and 290 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 368 µs and 995 ns  | NaN%       |         |
| Generate Foreign Traits    | 144 ms, 684 µs and 760 ns | NaN%       |         |
| Generate Insertable Traits | 357 ms, 676 µs and 858 ns | NaN%       |         |
| Generate Updatable Traits  | 49 ms, 972 µs and 515 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 175 ms, 285 µs and 12 ns (NaN% of all time).

| name                            | time                     | percentage | comment |
|---------------------------------|--------------------------|------------|---------|
| Procedure Impl Codegen          | 12 ms, 117 µs and 305 ns | NaN%       |         |
| procedure template Impl Codegen | 175 ms, 285 µs and 12 ns | NaN%       |         |

![Plot](time_requirements_report.png)
