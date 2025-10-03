# Time Report for Building Core Structures

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB` which took 6 minutes, 4 seconds, 589 ms, 782 µs and 88 ns (94.55% of all time).

| name                                      | time                                           | percentage | comment |
|-------------------------------------------|------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 259 ms, 148 µs and 1 ns             | 0.78%      |         |
| Init DB                                   | 6 minutes, 4 seconds, 589 ms, 782 µs and 88 ns | 94.55%     |         |
| Code Generation                           | 17 seconds, 482 ms, 892 µs and 157 ns          | 4.42%      |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 4 seconds, 589 ms, 554 µs and 923 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 227 µs and 165 ns                               | 0.00%      |         |
| Init DB Transaction | 6 minutes, 4 seconds, 589 ms, 554 µs and 923 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 57 seconds, 788 ms, 401 µs and 697 ns (98.08% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 114 ms, 407 µs and 959 ns             | 1.65%      |         |
| Initialize Migrations         | 686 ms, 745 µs and 267 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 57 seconds, 788 ms, 401 µs and 697 ns | 98.08%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 48 seconds, 418 ms, 666 µs and 671 ns (97.48% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 48 seconds, 418 ms, 666 µs and 671 ns | 97.48%     |         |
| Check constraints in schema 'public'               | 8 seconds, 328 ms, 603 µs and 187 ns             | 2.24%      |         |
| Procedure and procedure template check constraints | 1 second, 41 ms, 131 µs and 839 ns               | 0.28%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 209 ms, 160 µs and 96 ns (100.00% of all time).

| name                                     | time                                | percentage | comment |
|------------------------------------------|-------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 209 ms, 160 µs and 96 ns | 100.00%    |         |
| Lowercase column and table names         | 86 ms, 473 µs and 57 ns             | 0.00%      |         |
| Standard column names and types          | 18 ms, 298 µs and 331 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 11 ms, 533 µs and 165 ns            | 0.00%      |         |
| Word deprecation constraints             | 3 ms, 138 µs and 538 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Code generation` which took 16 seconds, 618 ms, 37 µs and 940 ns (94.12% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 16 seconds, 618 ms, 37 µs and 940 ns | 94.12%     |         |
| Procedure Codegen | 864 ms, 854 µs and 217 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was 16 seconds.
The slowest task was `Generate Structs` which took 15 seconds, 616 ms, 868 µs and 359 ns (93.75% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 226 µs and 235 ns                     | 0.00%      |         |
| Creating table extension network | 690 µs and 347 ns                     | 0.00%      |         |
| Generating Diesel code           | 56 ms, 513 µs and 155 ns              | 0.00%      |         |
| Generate Structs                 | 15 seconds, 616 ms, 868 µs and 359 ns | 93.75%     |         |
| Generate Web Common Traits       | 943 ms, 739 µs and 844 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 28 ms, 625 µs and 417 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 28 ms, 625 µs and 417 ns | NaN%       |         |
| Generating types schema                                | 6 ms, 686 µs and 955 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 21 ms, 200 µs and 783 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 15 seconds.
The slowest task was `Generate Table Structs` which took 15 seconds, 611 ms, 472 µs and 656 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 395 µs and 703 ns               | 0.00%      |         |
| Generate Table Structs | 15 seconds, 611 ms, 472 µs and 656 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 937 ms, 519 µs and 771 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 6 ms, 220 µs and 73 ns    | NaN%       |         |
| Generate Table Traits | 937 ms, 519 µs and 771 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 545 ms, 549 µs and 811 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 22 ms, 363 µs and 752 ns  | NaN%       |         |
| Generate Deletable Traits  | 47 ms, 200 µs and 624 ns  | NaN%       |         |
| Generate Upsertable Traits | 53 ms, 511 µs and 690 ns  | NaN%       |         |
| Generate Foreign Traits    | 217 ms, 283 µs and 851 ns | NaN%       |         |
| Generate Insertable Traits | 545 ms, 549 µs and 811 ns | NaN%       |         |
| Generate Updatable Traits  | 51 ms, 610 µs and 43 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 814 ms, 704 µs and 850 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 50 ms, 149 µs and 367 ns  | NaN%       |         |
| procedure template Impl Codegen | 814 ms, 704 µs and 850 ns | NaN%       |         |

![Plot](time_requirements_report.png)
