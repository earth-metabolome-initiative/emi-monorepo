# Time Report for Building Core Structures

The total time spent on all tasks was 7 minutes.
The slowest task was `Init DB` which took 6 minutes, 41 seconds, 188 ms, 846 µs and 58 ns (95.02% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 252 ms, 829 µs and 39 ns             | 0.71%      |         |
| Init DB                                   | 6 minutes, 41 seconds, 188 ms, 846 µs and 58 ns | 95.02%     |         |
| Code Generation                           | 18 seconds, 224 ms, 855 µs and 328 ns           | 4.27%      |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 41 seconds, 188 ms, 639 µs and 613 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 206 µs and 445 ns                                | 0.00%      |         |
| Init DB Transaction | 6 minutes, 41 seconds, 188 ms, 639 µs and 613 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 6 minutes, 34 seconds, 344 ms, 657 µs and 312 ns (98.25% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 131 ms, 312 µs and 830 ns             | 1.50%      |         |
| Initialize Migrations         | 712 ms, 669 µs and 471 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 6 minutes, 34 seconds, 344 ms, 657 µs and 312 ns | 98.25%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 6 minutes.
The slowest task was `Procedure and procedure template alignment` which took 6 minutes, 24 seconds, 561 ms, 803 µs and 952 ns (97.46% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 6 minutes, 24 seconds, 561 ms, 803 µs and 952 ns | 97.46%     |         |
| Check constraints in schema 'public'               | 8 seconds, 618 ms, 540 µs and 705 ns             | 2.03%      |         |
| Procedure and procedure template check constraints | 1 second, 164 ms, 312 µs and 655 ns              | 0.25%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 491 ms, 923 µs and 605 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 491 ms, 923 µs and 605 ns | 100.00%    |         |
| Lowercase column and table names         | 91 ms, 705 µs and 962 ns             | 0.00%      |         |
| Standard column names and types          | 18 ms, 398 µs and 105 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 12 ms, 874 µs and 991 ns             | 0.00%      |         |
| Word deprecation constraints             | 3 ms, 638 µs and 42 ns               | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 18 seconds.
The slowest task was `Code generation` which took 17 seconds, 204 ms, 429 µs and 445 ns (94.44% of all time).

| name              | time                                  | percentage | comment |
|-------------------|---------------------------------------|------------|---------|
| Code generation   | 17 seconds, 204 ms, 429 µs and 445 ns | 94.44%     |         |
| Procedure Codegen | 1 second, 20 ms, 425 µs and 883 ns    | 5.56%      |         |

### Time Report for Code generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Generate Structs` which took 16 seconds, 175 ms, 387 µs and 752 ns (94.12% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 242 µs and 665 ns                     | 0.00%      |         |
| Creating table extension network | 762 µs and 57 ns                      | 0.00%      |         |
| Generating Diesel code           | 58 ms, 601 µs and 694 ns              | 0.00%      |         |
| Generate Structs                 | 16 seconds, 175 ms, 387 µs and 752 ns | 94.12%     |         |
| Generate Web Common Traits       | 969 ms, 435 µs and 277 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 29 ms, 505 µs and 117 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 29 ms, 505 µs and 117 ns | NaN%       |         |
| Generating types schema                                | 7 ms, 289 µs and 445 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 21 ms, 807 µs and 132 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 16 seconds.
The slowest task was `Generate Table Structs` which took 16 seconds, 169 ms, 674 µs and 743 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 713 µs and 9 ns                 | 0.00%      |         |
| Generate Table Structs | 16 seconds, 169 ms, 674 µs and 743 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 964 ms, 691 µs and 19 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 4 ms, 744 µs and 258 ns  | NaN%       |         |
| Generate Table Traits | 964 ms, 691 µs and 19 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 569 ms, 164 µs and 138 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 22 ms, 605 µs and 491 ns  | NaN%       |         |
| Generate Deletable Traits  | 46 ms, 432 µs and 340 ns  | NaN%       |         |
| Generate Upsertable Traits | 54 ms, 881 µs and 960 ns  | NaN%       |         |
| Generate Foreign Traits    | 220 ms, 217 µs and 969 ns | NaN%       |         |
| Generate Insertable Traits | 569 ms, 164 µs and 138 ns | NaN%       |         |
| Generate Updatable Traits  | 51 ms, 389 µs and 121 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 966 ms, 210 µs and 567 ns (0.00% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 54 ms, 215 µs and 316 ns  | 0.00%      |         |
| procedure template Impl Codegen | 966 ms, 210 µs and 567 ns | 0.00%      |         |

![Plot](time_requirements_report.png)
