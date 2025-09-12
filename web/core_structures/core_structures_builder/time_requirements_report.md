# Time Report for Building Core Structures

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 30 seconds, 89 ms, 44 µs and 767 ns (94.29% of all time).

| name                                      | time                                           | percentage | comment |
|-------------------------------------------|------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 208 ms, 102 µs and 174 ns           | 0.86%      |         |
| Init DB                                   | 5 minutes, 30 seconds, 89 ms, 44 µs and 767 ns | 94.29%     |         |
| Code Generation                           | 17 seconds, 696 ms, 373 µs and 619 ns          | 4.86%      |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 30 seconds, 88 ms, 823 µs and 772 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 220 µs and 995 ns                               | 0.00%      |         |
| Init DB Transaction | 5 minutes, 30 seconds, 88 ms, 823 µs and 772 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 23 seconds, 323 ms, 764 µs and 258 ns (97.88% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 84 ms, 525 µs and 438 ns              | 1.82%      |         |
| Initialize Migrations         | 680 ms, 534 µs and 76 ns                         | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 23 seconds, 323 ms, 764 µs and 258 ns | 97.88%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 13 seconds, 887 ms, 834 µs and 187 ns (96.90% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 13 seconds, 887 ms, 834 µs and 187 ns | 96.90%     |         |
| Check constraints in schema 'public'               | 8 seconds, 440 ms, 209 µs and 701 ns             | 2.48%      |         |
| Procedure and procedure template check constraints | 995 ms, 720 µs and 370 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 324 ms, 27 µs and 994 ns (100.00% of all time).

| name                                     | time                                | percentage | comment |
|------------------------------------------|-------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 324 ms, 27 µs and 994 ns | 100.00%    |         |
| Lowercase column and table names         | 85 ms, 607 µs and 796 ns            | 0.00%      |         |
| Standard column names and types          | 16 ms, 303 µs and 744 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 11 ms, 308 µs and 789 ns            | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 961 µs and 378 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Code generation` which took 17 seconds, 416 ms, 305 µs and 979 ns (100.00% of all time).

| name              | time                                  | percentage | comment |
|-------------------|---------------------------------------|------------|---------|
| Code generation   | 17 seconds, 416 ms, 305 µs and 979 ns | 100.00%    |         |
| Procedure Codegen | 280 ms, 67 µs and 640 ns              | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Generate Structs` which took 16 seconds, 444 ms, 147 µs and 202 ns (94.12% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 215 µs and 435 ns                     | 0.00%      |         |
| Creating table extension network | 658 µs and 326 ns                     | 0.00%      |         |
| Generating Diesel code           | 54 ms, 705 µs and 335 ns              | 0.00%      |         |
| Generate Structs                 | 16 seconds, 444 ms, 147 µs and 202 ns | 94.12%     |         |
| Generate Web Common Traits       | 916 ms, 579 µs and 681 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 28 ms, 577 µs and 136 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 28 ms, 577 µs and 136 ns | NaN%       |         |
| Generating types schema                                | 6 ms, 405 µs and 257 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 19 ms, 722 µs and 942 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 16 seconds.
The slowest task was `Generate Table Structs` which took 16 seconds, 439 ms, 125 µs and 987 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 21 µs and 215 ns                | 0.00%      |         |
| Generate Table Structs | 16 seconds, 439 ms, 125 µs and 987 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 912 ms, 988 µs and 359 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 591 µs and 322 ns   | NaN%       |         |
| Generate Table Traits | 912 ms, 988 µs and 359 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 510 ms, 157 µs and 361 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 21 ms, 474 µs and 773 ns  | NaN%       |         |
| Generate Deletable Traits  | 47 ms, 729 µs and 786 ns  | NaN%       |         |
| Generate Upsertable Traits | 51 ms, 224 µs and 256 ns  | NaN%       |         |
| Generate Foreign Traits    | 208 ms, 316 µs and 82 ns  | NaN%       |         |
| Generate Insertable Traits | 510 ms, 157 µs and 361 ns | NaN%       |         |
| Generate Updatable Traits  | 74 ms, 86 µs and 101 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 258 ms, 253 µs and 189 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 21 ms, 814 µs and 451 ns  | NaN%       |         |
| procedure template Impl Codegen | 258 ms, 253 µs and 189 ns | NaN%       |         |

![Plot](time_requirements_report.png)
