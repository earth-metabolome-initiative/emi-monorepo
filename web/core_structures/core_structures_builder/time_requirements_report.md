# Time Report for Building Core Structures

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 15 seconds, 515 ms, 189 µs and 367 ns (91.55% of all time).

| name                                      | time                                             | percentage | comment |
|-------------------------------------------|--------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 7 seconds, 774 ms, 501 µs and 270 ns             | 3.29%      |         |
| Init DB                                   | 3 minutes, 15 seconds, 515 ms, 189 µs and 367 ns | 91.55%     |         |
| Code Generation                           | 10 seconds, 340 ms, 743 µs and 345 ns            | 4.69%      |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 15 seconds, 515 ms, 56 µs and 856 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 132 µs and 511 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 15 seconds, 515 ms, 56 µs and 856 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 3 minutes, 10 seconds, 940 ms, 308 µs and 621 ns (97.44% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 74 ms, 246 µs and 244 ns              | 2.05%      |         |
| Initialize Migrations         | 500 ms, 501 µs and 991 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 3 minutes, 10 seconds, 940 ms, 308 µs and 621 ns | 97.44%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 3 minutes.
The slowest task was `Procedure and procedure template alignment` which took 3 minutes, 5 seconds, 491 ms, 833 µs and 23 ns (97.37% of all time).

| name                                               | time                                           | percentage | comment |
|----------------------------------------------------|------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 3 minutes, 5 seconds, 491 ms, 833 µs and 23 ns | 97.37%     |         |
| Check constraints in schema 'public'               | 5 seconds, 62 ms, 406 µs and 659 ns            | 2.63%      |         |
| Procedure and procedure template check constraints | 386 ms, 68 µs and 939 ns                       | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 997 ms, 972 µs and 898 ns (80.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 997 ms, 972 µs and 898 ns | 80.00%     |         |
| Lowercase column and table names         | 52 ms, 283 µs and 141 ns             | 0.00%      |         |
| Standard column names and types          | 6 ms, 813 µs and 233 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 4 ms, 336 µs and 921 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms and 466 ns                      | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 10 seconds, 153 ms, 352 µs and 761 ns (100.00% of all time).

| name              | time                                  | percentage | comment |
|-------------------|---------------------------------------|------------|---------|
| Code generation   | 10 seconds, 153 ms, 352 µs and 761 ns | 100.00%    |         |
| Procedure Codegen | 187 ms, 390 µs and 584 ns             | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 9 seconds, 469 ms, 234 µs and 833 ns (90.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 307 µs and 166 ns                    | 0.00%      |         |
| Creating table extension network | 614 µs and 120 ns                    | 0.00%      |         |
| Generating Diesel code           | 35 ms, 606 µs and 236 ns             | 0.00%      |         |
| Generate Structs                 | 9 seconds, 469 ms, 234 µs and 833 ns | 90.00%     |         |
| Generate Web Common Traits       | 647 ms, 590 µs and 406 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 16 ms, 720 µs and 593 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 16 ms, 720 µs and 593 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 838 µs and 245 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 15 ms, 47 µs and 398 ns  | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 9 seconds, 465 ms, 805 µs and 387 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 429 µs and 446 ns              | 0.00%      |         |
| Generate Table Structs | 9 seconds, 465 ms, 805 µs and 387 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 644 ms, 541 µs and 306 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 49 µs and 100 ns    | NaN%       |         |
| Generate Table Traits | 644 ms, 541 µs and 306 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 360 ms, 627 µs and 765 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 936 µs and 545 ns  | NaN%       |         |
| Generate Deletable Traits  | 37 ms, 64 µs and 74 ns    | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 320 µs and 481 ns  | NaN%       |         |
| Generate Foreign Traits    | 145 ms, 330 µs and 181 ns | NaN%       |         |
| Generate Insertable Traits | 360 ms, 627 µs and 765 ns | NaN%       |         |
| Generate Updatable Traits  | 50 ms, 262 µs and 260 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 175 ms, 466 µs and 859 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 923 µs and 725 ns  | NaN%       |         |
| procedure template Impl Codegen | 175 ms, 466 µs and 859 ns | NaN%       |         |

![Plot](time_requirements_report.png)
