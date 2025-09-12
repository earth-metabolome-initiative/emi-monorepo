# Time Report for Building Core Structures

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 2 seconds, 545 ms, 530 µs and 318 ns (91.00% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 171 ms, 221 µs and 964 ns            | 4.00%      |         |
| Init DB                                   | 3 minutes, 2 seconds, 545 ms, 530 µs and 318 ns | 91.00%     |         |
| Code Generation                           | 10 seconds, 241 ms, 920 µs and 217 ns           | 5.00%      |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 2 seconds, 545 ms, 254 µs and 481 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 275 µs and 837 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 2 seconds, 545 ms, 254 µs and 481 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 58 seconds, 507 ms, 199 µs and 607 ns (97.80% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 584 ms, 92 µs and 677 ns              | 1.65%      |         |
| Initialize Migrations         | 453 ms, 962 µs and 197 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 58 seconds, 507 ms, 199 µs and 607 ns | 97.80%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 53 seconds, 268 ms, 324 µs and 188 ns (97.19% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 53 seconds, 268 ms, 324 µs and 188 ns | 97.19%     |         |
| Check constraints in schema 'public'               | 4 seconds, 883 ms, 173 µs and 448 ns             | 2.25%      |         |
| Procedure and procedure template check constraints | 355 ms, 701 µs and 971 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 826 ms, 141 µs and 8 ns (100.00% of all time).

| name                                     | time                               | percentage | comment |
|------------------------------------------|------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 826 ms, 141 µs and 8 ns | 100.00%    |         |
| Lowercase column and table names         | 45 ms, 877 µs and 833 ns           | 0.00%      |         |
| Standard column names and types          | 5 ms, 976 µs and 573 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 4 ms, 152 µs and 360 ns            | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 25 µs and 674 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 10 seconds, 66 ms, 335 µs and 114 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 10 seconds, 66 ms, 335 µs and 114 ns | 100.00%    |         |
| Procedure Codegen | 175 ms, 585 µs and 103 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 9 seconds, 438 ms, 790 µs and 114 ns (90.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 243 µs and 158 ns                    | 0.00%      |         |
| Creating table extension network | 487 µs and 88 ns                     | 0.00%      |         |
| Generating Diesel code           | 30 ms, 10 µs and 128 ns              | 0.00%      |         |
| Generate Structs                 | 9 seconds, 438 ms, 790 µs and 114 ns | 90.00%     |         |
| Generate Web Common Traits       | 596 ms, 804 µs and 626 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 14 ms, 136 µs and 754 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 14 ms, 136 µs and 754 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 442 µs and 705 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 12 ms, 430 µs and 669 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 9 seconds, 435 ms, 839 µs and 876 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 950 µs and 238 ns              | 0.00%      |         |
| Generate Table Structs | 9 seconds, 435 ms, 839 µs and 876 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 594 ms, 416 µs and 108 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 388 µs and 518 ns   | NaN%       |         |
| Generate Table Traits | 594 ms, 416 µs and 108 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 333 ms, 951 µs and 736 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 398 µs and 795 ns  | NaN%       |         |
| Generate Deletable Traits  | 31 ms, 786 µs and 98 ns   | NaN%       |         |
| Generate Upsertable Traits | 32 ms, 194 µs and 366 ns  | NaN%       |         |
| Generate Foreign Traits    | 130 ms, 592 µs and 281 ns | NaN%       |         |
| Generate Insertable Traits | 333 ms, 951 µs and 736 ns | NaN%       |         |
| Generate Updatable Traits  | 52 ms, 492 µs and 832 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 163 ms, 674 µs and 141 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 910 µs and 962 ns  | NaN%       |         |
| procedure template Impl Codegen | 163 ms, 674 µs and 141 ns | NaN%       |         |

![Plot](time_requirements_report.png)
