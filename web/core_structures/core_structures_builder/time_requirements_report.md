# Time Report for Building Core Structures

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 29 seconds, 737 ms, 588 µs and 86 ns (94.00% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 254 ms, 551 µs and 329 ns            | 0.86%      |         |
| Init DB                                   | 5 minutes, 29 seconds, 737 ms, 588 µs and 86 ns | 94.00%     |         |
| Code Generation                           | 17 seconds, 584 ms, 569 µs and 373 ns           | 4.86%      |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 29 seconds, 737 ms, 449 µs and 543 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 138 µs and 543 ns                                | 0.00%      |         |
| Init DB Transaction | 5 minutes, 29 seconds, 737 ms, 449 µs and 543 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 23 seconds, 115 ms, 943 µs and 315 ns (98.18% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 5 seconds, 952 ms, 437 µs and 471 ns             | 1.52%      |         |
| Initialize Migrations         | 669 ms, 68 µs and 757 ns                         | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 23 seconds, 115 ms, 943 µs and 315 ns | 98.18%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 14 seconds, 182 ms, 777 µs and 577 ns (97.21% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 14 seconds, 182 ms, 777 µs and 577 ns | 97.21%     |         |
| Check constraints in schema 'public'               | 8 seconds, 90 ms, 653 µs and 358 ns              | 2.48%      |         |
| Procedure and procedure template check constraints | 842 ms, 512 µs and 380 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 7 seconds, 974 ms, 941 µs and 226 ns (87.50% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 7 seconds, 974 ms, 941 µs and 226 ns | 87.50%     |         |
| Lowercase column and table names         | 85 ms, 287 µs and 281 ns             | 0.00%      |         |
| Standard column names and types          | 16 ms, 969 µs and 441 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 10 ms, 681 µs and 436 ns             | 0.00%      |         |
| Word deprecation constraints             | 2 ms, 773 µs and 974 ns              | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Code generation` which took 16 seconds, 828 ms, 97 µs and 337 ns (94.12% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 16 seconds, 828 ms, 97 µs and 337 ns | 94.12%     |         |
| Procedure Codegen | 756 ms, 472 µs and 36 ns             | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was 16 seconds.
The slowest task was `Generate Structs` which took 15 seconds, 806 ms, 274 µs and 715 ns (93.75% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 1 ms, 6 µs and 424 ns                 | 0.00%      |         |
| Creating table extension network | 700 µs and 446 ns                     | 0.00%      |         |
| Generating Diesel code           | 55 ms, 4 µs and 276 ns                | 0.00%      |         |
| Generate Structs                 | 15 seconds, 806 ms, 274 µs and 715 ns | 93.75%     |         |
| Generate Web Common Traits       | 965 ms, 111 µs and 476 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 28 ms, 217 µs and 249 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 28 ms, 217 µs and 249 ns | NaN%       |         |
| Generating types schema                                | 6 ms, 701 µs and 494 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 20 ms, 85 µs and 533 ns  | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 15 seconds.
The slowest task was `Generate Table Structs` which took 15 seconds, 800 ms, 986 µs and 383 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 288 µs and 332 ns               | 0.00%      |         |
| Generate Table Structs | 15 seconds, 800 ms, 986 µs and 383 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 960 ms, 201 µs and 193 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 4 ms, 910 µs and 283 ns   | NaN%       |         |
| Generate Table Traits | 960 ms, 201 µs and 193 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 506 ms, 338 µs and 880 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 21 ms, 474 µs and 504 ns  | NaN%       |         |
| Generate Deletable Traits  | 133 ms, 839 µs and 360 ns | NaN%       |         |
| Generate Upsertable Traits | 50 ms, 530 µs and 32 ns   | NaN%       |         |
| Generate Foreign Traits    | 204 ms, 419 µs and 94 ns  | NaN%       |         |
| Generate Insertable Traits | 506 ms, 338 µs and 880 ns | NaN%       |         |
| Generate Updatable Traits  | 43 ms, 599 µs and 323 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 710 ms, 480 µs and 357 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 45 ms, 991 µs and 679 ns  | NaN%       |         |
| procedure template Impl Codegen | 710 ms, 480 µs and 357 ns | NaN%       |         |

![Plot](time_requirements_report.png)
