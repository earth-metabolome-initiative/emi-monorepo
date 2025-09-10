# Time Report for Building Core Structures

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 24 seconds, 347 ms, 703 µs and 734 ns (94.46% of all time).

| name                                      | time                                             | percentage | comment |
|-------------------------------------------|--------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 261 ms, 77 µs and 93 ns               | 0.87%      |         |
| Init DB                                   | 5 minutes, 24 seconds, 347 ms, 703 µs and 734 ns | 94.46%     |         |
| Code Generation                           | 15 seconds, 913 ms, 598 µs and 416 ns            | 4.37%      |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 24 seconds, 347 ms, 479 µs and 439 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 224 µs and 295 ns                                | 0.00%      |         |
| Init DB Transaction | 5 minutes, 24 seconds, 347 ms, 479 µs and 439 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 17 seconds, 663 ms, 220 µs and 523 ns (97.84% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 30 ms, 471 µs and 139 ns              | 1.85%      |         |
| Initialize Migrations         | 653 ms, 787 µs and 777 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 17 seconds, 663 ms, 220 µs and 523 ns | 97.84%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 8 seconds, 530 ms, 749 µs and 977 ns (97.16% of all time).

| name                                               | time                                            | percentage | comment |
|----------------------------------------------------|-------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 8 seconds, 530 ms, 749 µs and 977 ns | 97.16%     |         |
| Check constraints in schema 'public'               | 8 seconds, 136 ms, 6 µs and 525 ns              | 2.52%      |         |
| Procedure and procedure template check constraints | 996 ms, 464 µs and 21 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 21 ms, 907 µs and 273 ns (100.00% of all time).

| name                                     | time                                | percentage | comment |
|------------------------------------------|-------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 21 ms, 907 µs and 273 ns | 100.00%    |         |
| Lowercase column and table names         | 84 ms, 198 µs and 308 ns            | 0.00%      |         |
| Standard column names and types          | 16 ms, 190 µs and 955 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 10 ms, 855 µs and 614 ns            | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 854 µs and 375 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Code generation` which took 15 seconds, 652 ms, 57 µs and 760 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 15 seconds, 652 ms, 57 µs and 760 ns | 100.00%    |         |
| Procedure Codegen | 261 ms, 540 µs and 656 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Generate Structs` which took 14 seconds, 700 ms, 711 µs and 838 ns (93.33% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 267 µs and 397 ns                     | 0.00%      |         |
| Creating table extension network | 675 µs and 308 ns                     | 0.00%      |         |
| Generating Diesel code           | 53 ms, 828 µs and 280 ns              | 0.00%      |         |
| Generate Structs                 | 14 seconds, 700 ms, 711 µs and 838 ns | 93.33%     |         |
| Generate Web Common Traits       | 896 ms, 574 µs and 937 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 27 ms, 874 µs and 380 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 27 ms, 874 µs and 380 ns | NaN%       |         |
| Generating types schema                                | 6 ms, 178 µs and 82 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 19 ms, 775 µs and 818 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 14 seconds.
The slowest task was `Generate Table Structs` which took 14 seconds, 695 ms, 852 µs and 761 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 4 ms, 859 µs and 77 ns                | 0.00%      |         |
| Generate Table Structs | 14 seconds, 695 ms, 852 µs and 761 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 892 ms, 614 µs and 633 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 960 µs and 304 ns   | NaN%       |         |
| Generate Table Traits | 892 ms, 614 µs and 633 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 498 ms, 888 µs and 342 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 20 ms, 711 µs and 522 ns  | NaN%       |         |
| Generate Deletable Traits  | 48 ms, 343 µs and 33 ns   | NaN%       |         |
| Generate Upsertable Traits | 48 ms, 956 µs and 199 ns  | NaN%       |         |
| Generate Foreign Traits    | 203 ms, 622 µs and 533 ns | NaN%       |         |
| Generate Insertable Traits | 498 ms, 888 µs and 342 ns | NaN%       |         |
| Generate Updatable Traits  | 72 ms, 93 µs and 4 ns     | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 245 ms, 338 µs and 933 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 16 ms, 201 µs and 723 ns  | NaN%       |         |
| procedure template Impl Codegen | 245 ms, 338 µs and 933 ns | NaN%       |         |

![Plot](time_requirements_report.png)
