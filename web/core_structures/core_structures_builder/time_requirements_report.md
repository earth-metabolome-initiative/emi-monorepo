# Time Report for Building Core Structures

The total time spent on all tasks was 19 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 82 ms, 859 µs and 687 ns (42.11% of all time).

| name                                      | time                                | percentage | comment |
|-------------------------------------------|-------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 82 ms, 859 µs and 687 ns | 42.11%     |         |
| Init DB                                   | 4 seconds, 368 ms, 98 µs and 871 ns | 21.05%     |         |
| Code Generation                           | 6 seconds, 738 ms, 59 µs and 155 ns | 31.58%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 367 ms, 953 µs and 130 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 145 µs and 741 ns                    | 0.00%      |         |
| Init DB Transaction | 4 seconds, 367 ms, 953 µs and 130 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 885 ms, 277 µs and 121 ns (75.00% of all time).

| name                  | time                                 | percentage | comment |
|-----------------------|--------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 885 ms, 277 µs and 121 ns | 75.00%     |         |
| Initialize Migrations | 482 ms, 676 µs and 9 ns              | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 568 ms, 211 µs and 927 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 568 ms, 211 µs and 927 ns | 100.00%    |         |
| Procedure Codegen | 169 ms, 847 µs and 228 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 685 ms, 20 µs and 958 ns (83.33% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 11 ms, 797 µs and 911 ns            | 0.00%      |         |
| Creating table extension network | 16 ms, 169 µs and 61 ns             | 0.00%      |         |
| Generating Diesel code           | 155 ms, 887 µs and 262 ns           | 0.00%      |         |
| Generate Structs                 | 5 seconds, 685 ms, 20 µs and 958 ns | 83.33%     |         |
| Generate Web Common Traits       | 699 ms, 336 µs and 735 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 96 ms, 957 µs and 1 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 55 ms, 139 µs and 840 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 790 µs and 421 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 96 ms, 957 µs and 1 ns   | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 681 ms, 596 µs and 240 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 424 µs and 718 ns              | 0.00%      |         |
| Generate Table Structs | 5 seconds, 681 ms, 596 µs and 240 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 696 ms, 871 µs and 531 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 465 µs and 204 ns   | NaN%       |         |
| Generate Table Traits | 696 ms, 871 µs and 531 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 400 ms, 977 µs and 380 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 14 ms, 310 µs and 756 ns  | NaN%       |         |
| Generate Deletable Traits  | 50 ms, 559 µs and 342 ns  | NaN%       |         |
| Generate Upsertable Traits | 34 ms, 141 µs and 294 ns  | NaN%       |         |
| Generate Foreign Traits    | 138 ms, 524 µs and 293 ns | NaN%       |         |
| Generate Insertable Traits | 400 ms, 977 µs and 380 ns | NaN%       |         |
| Generate Updatable Traits  | 58 ms, 358 µs and 466 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 157 ms, 533 µs and 777 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 12 ms, 313 µs and 451 ns  | NaN%       |         |
| procedure template Impl Codegen | 157 ms, 533 µs and 777 ns | NaN%       |         |

![Plot](time_requirements_report.png)
