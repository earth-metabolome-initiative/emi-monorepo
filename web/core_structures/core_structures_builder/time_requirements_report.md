# Time Report for Building Core Structures

The total time spent on all tasks was 18 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 767 µs and 34 ns (44.44% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 767 µs and 34 ns          | 44.44%     |         |
| Init DB                                   | 4 seconds, 321 ms, 192 µs and 191 ns | 22.22%     |         |
| Code Generation                           | 6 seconds, 282 ms, 469 µs and 203 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 321 ms, 81 µs and 694 ns (100.00% of all time).

| name                | time                                | percentage | comment |
|---------------------|-------------------------------------|------------|---------|
| Retrieve CSVs       | 110 µs and 497 ns                   | 0.00%      |         |
| Init DB Transaction | 4 seconds, 321 ms, 81 µs and 694 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 829 ms, 46 µs and 33 ns (75.00% of all time).

| name                  | time                               | percentage | comment |
|-----------------------|------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 829 ms, 46 µs and 33 ns | 75.00%     |         |
| Initialize Migrations | 492 ms, 35 µs and 661 ns           | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 115 ms, 311 µs and 580 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 115 ms, 311 µs and 580 ns | 100.00%    |         |
| Procedure Codegen | 167 ms, 157 µs and 623 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 267 ms, 935 µs and 489 ns (83.33% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 11 ms, 167 µs and 560 ns             | 0.00%      |         |
| Creating table extension network | 20 ms, 237 µs and 382 ns             | 0.00%      |         |
| Generating Diesel code           | 131 ms, 428 µs and 459 ns            | 0.00%      |         |
| Generate Structs                 | 5 seconds, 267 ms, 935 µs and 489 ns | 83.33%     |         |
| Generate Web Common Traits       | 684 ms, 542 µs and 690 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 74 ms, 56 µs and 934 ns (NaN% of all time).

| name                                                   | time                    | percentage | comment |
|--------------------------------------------------------|-------------------------|------------|---------|
| Generating tables schema                               | 52 ms, 559 µs and 63 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 812 µs and 462 ns | NaN%       |         |
| Generating allow tables to appear in same query schema | 74 ms, 56 µs and 934 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 264 ms, 767 µs and 649 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 167 µs and 840 ns              | 0.00%      |         |
| Generate Table Structs | 5 seconds, 264 ms, 767 µs and 649 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 682 ms, 4 µs and 234 ns (NaN% of all time).

| name                  | time                    | percentage | comment |
|-----------------------|-------------------------|------------|---------|
| Generate Types Traits | 2 ms, 538 µs and 456 ns | NaN%       |         |
| Generate Table Traits | 682 ms, 4 µs and 234 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 386 ms, 936 µs and 293 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 12 ms, 697 µs and 689 ns  | NaN%       |         |
| Generate Deletable Traits  | 59 ms, 768 µs and 95 ns   | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 573 µs and 898 ns  | NaN%       |         |
| Generate Foreign Traits    | 135 ms, 967 µs and 927 ns | NaN%       |         |
| Generate Insertable Traits | 386 ms, 936 µs and 293 ns | NaN%       |         |
| Generate Updatable Traits  | 51 ms, 60 µs and 332 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 156 ms, 480 µs and 155 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 10 ms, 677 µs and 468 ns  | NaN%       |         |
| procedure template Impl Codegen | 156 ms, 480 µs and 155 ns | NaN%       |         |

![Plot](time_requirements_report.png)
