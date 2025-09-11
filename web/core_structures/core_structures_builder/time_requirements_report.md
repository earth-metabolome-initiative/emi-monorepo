# Time Report for Building Core Structures

The total time spent on all tasks was 21 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 530 ms, 277 µs and 864 ns (38.10% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 530 ms, 277 µs and 864 ns | 38.10%     |         |
| Init DB                                   | 5 seconds, 822 ms, 27 µs and 335 ns  | 23.81%     |         |
| Code Generation                           | 6 seconds, 904 ms, 963 µs and 530 ns | 28.57%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 821 ms, 922 µs and 386 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 104 µs and 949 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 821 ms, 922 µs and 386 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 4 seconds, 1 ms, 400 µs and 680 ns (80.00% of all time).

| name                          | time                                | percentage | comment |
|-------------------------------|-------------------------------------|------------|---------|
| Initialize CSVs               | 4 seconds, 1 ms, 400 µs and 680 ns  | 80.00%     |         |
| Initialize Migrations         | 507 ms, 728 µs and 900 ns           | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 312 ms, 792 µs and 806 ns | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 756 ms, 101 µs and 148 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 756 ms, 101 µs and 148 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 151 ms, 309 µs and 185 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 405 ms, 382 µs and 473 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 137 ms, 796 µs and 792 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 137 ms, 796 µs and 792 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 213 µs and 929 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 650 µs and 967 ns   | NaN%       |         |
| Not-null constraints on standard columns | 4 ms, 609 µs and 484 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 38 µs and 13 ns     | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 710 ms, 17 µs and 805 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 710 ms, 17 µs and 805 ns | 100.00%    |         |
| Procedure Codegen | 194 ms, 945 µs and 725 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 36 ms, 824 µs and 633 ns (100.00% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 147 µs and 23 ns                    | 0.00%      |         |
| Creating table extension network | 14 ms, 2 µs and 496 ns              | 0.00%      |         |
| Generating Diesel code           | 28 ms, 600 µs and 657 ns            | 0.00%      |         |
| Generate Structs                 | 6 seconds, 36 ms, 824 µs and 633 ns | 100.00%    |         |
| Generate Web Common Traits       | 630 ms, 442 µs and 996 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 425 µs and 568 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 10 ms, 316 µs and 412 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 858 µs and 677 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 425 µs and 568 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 33 ms, 707 µs and 250 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 117 µs and 383 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 33 ms, 707 µs and 250 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 626 ms, 64 µs and 582 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 4 ms, 378 µs and 414 ns  | NaN%       |         |
| Generate Table Traits | 626 ms, 64 µs and 582 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 343 ms, 995 µs and 937 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 17 ms, 256 µs and 858 ns  | NaN%       |         |
| Generate Deletable Traits  | 34 ms, 875 µs and 113 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 495 µs and 834 ns  | NaN%       |         |
| Generate Foreign Traits    | 144 ms, 155 µs and 6 ns   | NaN%       |         |
| Generate Insertable Traits | 343 ms, 995 µs and 937 ns | NaN%       |         |
| Generate Updatable Traits  | 50 ms, 285 µs and 834 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 183 ms, 556 µs and 157 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 389 µs and 568 ns  | NaN%       |         |
| procedure template Impl Codegen | 183 ms, 556 µs and 157 ns | NaN%       |         |

![Plot](time_requirements_report.png)
