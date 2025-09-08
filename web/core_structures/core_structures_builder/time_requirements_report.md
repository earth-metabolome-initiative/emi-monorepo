# Time Report for Building Core Structures

The total time spent on all tasks was 18 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 7 seconds, 382 ms, 351 µs and 239 ns (38.89% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 7 seconds, 382 ms, 351 µs and 239 ns | 38.89%     |         |
| Init DB                                   | 4 seconds, 448 ms, 607 µs and 396 ns | 22.22%     |         |
| Code Generation                           | 6 seconds, 637 ms, 858 µs and 856 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 448 ms, 495 µs and 396 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 112 µs                               | 0.00%      |         |
| Init DB Transaction | 4 seconds, 448 ms, 495 µs and 396 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 954 ms, 53 µs and 213 ns (75.00% of all time).

| name                  | time                                | percentage | comment |
|-----------------------|-------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 954 ms, 53 µs and 213 ns | 75.00%     |         |
| Initialize Migrations | 494 ms, 442 µs and 183 ns           | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 472 ms, 30 µs and 649 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 472 ms, 30 µs and 649 ns | 100.00%    |         |
| Procedure Codegen | 165 ms, 828 µs and 207 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 531 ms, 724 µs and 547 ns (83.33% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 13 ms, 349 µs and 157 ns             | 0.00%      |         |
| Creating table extension network | 19 ms, 517 µs and 462 ns             | 0.00%      |         |
| Generating Diesel code           | 179 ms, 334 µs and 582 ns            | 0.00%      |         |
| Generate Structs                 | 5 seconds, 531 ms, 724 µs and 547 ns | 83.33%     |         |
| Generate Web Common Traits       | 728 ms, 104 µs and 901 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 97 ms, 906 µs and 324 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 77 ms, 324 µs and 720 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 103 µs and 538 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 97 ms, 906 µs and 324 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 528 ms, 755 µs and 68 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 969 µs and 479 ns             | 0.00%      |         |
| Generate Table Structs | 5 seconds, 528 ms, 755 µs and 68 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 725 ms, 115 µs and 371 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 989 µs and 530 ns   | NaN%       |         |
| Generate Table Traits | 725 ms, 115 µs and 371 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 418 ms, 110 µs and 587 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 18 ms, 795 µs and 197 ns  | NaN%       |         |
| Generate Deletable Traits  | 64 ms, 168 µs and 276 ns  | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 871 µs and 917 ns  | NaN%       |         |
| Generate Foreign Traits    | 138 ms, 842 µs and 340 ns | NaN%       |         |
| Generate Insertable Traits | 418 ms, 110 µs and 587 ns | NaN%       |         |
| Generate Updatable Traits  | 48 ms, 327 µs and 54 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 154 ms, 734 µs and 979 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 93 µs and 228 ns   | NaN%       |         |
| procedure template Impl Codegen | 154 ms, 734 µs and 979 ns | NaN%       |         |

![Plot](time_requirements_report.png)
