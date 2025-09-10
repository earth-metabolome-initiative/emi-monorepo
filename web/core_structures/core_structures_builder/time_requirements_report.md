# Time Report for Building Core Structures

The total time spent on all tasks was 22 seconds.
The slowest task was `Code Generation` which took 10 seconds, 386 ms and 295 ns (45.45% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 2 seconds, 737 ms, 200 µs and 862 ns | 9.09%      |         |
| Init DB                                   | 9 seconds, 223 ms, 494 µs and 146 ns | 40.91%     |         |
| Code Generation                           | 10 seconds, 386 ms and 295 ns        | 45.45%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 9 seconds, 218 ms, 557 µs and 493 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 4 ms, 936 µs and 653 ns              | 0.00%      |         |
| Init DB Transaction | 9 seconds, 218 ms, 557 µs and 493 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 6 seconds, 225 ms, 33 µs and 561 ns (66.67% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 225 ms, 33 µs and 561 ns  | 66.67%     |         |
| Initialize Migrations         | 657 ms, 832 µs and 947 ns            | 0.00%      |         |
| Consistency Constraint Checks | 2 seconds, 335 ms, 690 µs and 985 ns | 22.22%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 1 second, 313 ms, 301 µs and 801 ns (50.00% of all time).

| name                                               | time                                | percentage | comment |
|----------------------------------------------------|-------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 1 second, 313 ms, 301 µs and 801 ns | 50.00%     |         |
| Check constraints in schema 'public'               | 161 ms, 641 µs and 748 ns           | 0.00%      |         |
| Procedure and procedure template check constraints | 860 ms, 747 µs and 436 ns           | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 144 ms, 439 µs and 426 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 144 ms, 439 µs and 426 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 495 µs and 764 ns   | NaN%       |         |
| Standard column names and types          | 8 ms, 640 µs and 86 ns    | NaN%       |         |
| Not-null constraints on standard columns | 5 ms, 576 µs and 578 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 489 µs and 894 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 10 seconds, 122 ms, 694 µs and 188 ns (100.00% of all time).

| name              | time                                  | percentage | comment |
|-------------------|---------------------------------------|------------|---------|
| Code generation   | 10 seconds, 122 ms, 694 µs and 188 ns | 100.00%    |         |
| Procedure Codegen | 263 ms, 306 µs and 107 ns             | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 9 seconds, 192 ms, 827 µs and 523 ns (90.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 257 µs and 516 ns                    | 0.00%      |         |
| Creating table extension network | 6 ms, 894 µs and 467 ns              | 0.00%      |         |
| Generating Diesel code           | 41 ms, 65 µs and 47 ns               | 0.00%      |         |
| Generate Structs                 | 9 seconds, 192 ms, 827 µs and 523 ns | 90.00%     |         |
| Generate Web Common Traits       | 881 ms, 649 µs and 635 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 19 ms, 795 µs and 462 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 13 ms, 431 µs and 906 ns | NaN%       |         |
| Generating types schema                                | 7 ms, 837 µs and 679 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 19 ms, 795 µs and 462 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 9 seconds, 187 ms, 10 µs and 160 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 817 µs and 363 ns             | 0.00%      |         |
| Generate Table Structs | 9 seconds, 187 ms, 10 µs and 160 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 876 ms, 461 µs and 156 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 5 ms, 188 µs and 479 ns   | NaN%       |         |
| Generate Table Traits | 876 ms, 461 µs and 156 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 484 ms, 717 µs and 969 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 21 ms, 475 µs and 950 ns  | NaN%       |         |
| Generate Deletable Traits  | 46 ms, 221 µs and 984 ns  | NaN%       |         |
| Generate Upsertable Traits | 50 ms, 265 µs and 487 ns  | NaN%       |         |
| Generate Foreign Traits    | 204 ms, 466 µs and 275 ns | NaN%       |         |
| Generate Insertable Traits | 484 ms, 717 µs and 969 ns | NaN%       |         |
| Generate Updatable Traits  | 69 ms, 313 µs and 491 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 246 ms, 809 µs and 871 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 16 ms, 496 µs and 236 ns  | NaN%       |         |
| procedure template Impl Codegen | 246 ms, 809 µs and 871 ns | NaN%       |         |

![Plot](time_requirements_report.png)
