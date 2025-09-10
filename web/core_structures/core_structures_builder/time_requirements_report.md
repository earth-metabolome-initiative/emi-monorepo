# Time Report for Building Core Structures

The total time spent on all tasks was 20 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 190 ms, 786 µs and 427 ns (40.00% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 190 ms, 786 µs and 427 ns | 40.00%     |         |
| Init DB                                   | 5 seconds, 687 ms, 514 µs and 825 ns | 25.00%     |         |
| Code Generation                           | 6 seconds, 892 ms, 100 µs and 761 ns | 30.00%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 687 ms, 389 µs and 805 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 125 µs and 20 ns                     | 0.00%      |         |
| Init DB Transaction | 5 seconds, 687 ms, 389 µs and 805 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 887 ms, 317 µs and 315 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 887 ms, 317 µs and 315 ns | 60.00%     |         |
| Initialize Migrations         | 492 ms, 711 µs and 724 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 307 ms, 360 µs and 766 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 770 ms, 599 µs and 810 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 770 ms, 599 µs and 810 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 140 ms, 830 µs and 732 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 395 ms, 930 µs and 224 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 126 ms, 942 µs and 401 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 126 ms, 942 µs and 401 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 773 µs and 957 ns   | NaN%       |         |
| Standard column names and types          | 7 ms, 180 µs and 188 ns   | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 698 µs and 874 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 235 µs and 312 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 706 ms, 91 µs and 256 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 6 seconds, 706 ms, 91 µs and 256 ns | 100.00%    |         |
| Procedure Codegen | 186 ms, 9 µs and 505 ns             | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 39 ms, 363 µs and 765 ns (100.00% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 179 µs and 903 ns                   | 0.00%      |         |
| Creating table extension network | 13 ms, 225 µs and 135 ns            | 0.00%      |         |
| Generating Diesel code           | 27 ms, 552 µs and 71 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 39 ms, 363 µs and 765 ns | 100.00%    |         |
| Generate Web Common Traits       | 625 ms, 770 µs and 382 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 21 µs and 613 ns (NaN% of all time).

| name                                                   | time                    | percentage | comment |
|--------------------------------------------------------|-------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 462 µs and 265 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 68 µs and 193 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 21 µs and 613 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 36 ms, 452 µs and 645 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 911 µs and 120 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 36 ms, 452 µs and 645 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 623 ms, 4 µs and 392 ns (NaN% of all time).

| name                  | time                    | percentage | comment |
|-----------------------|-------------------------|------------|---------|
| Generate Types Traits | 2 ms, 765 µs and 990 ns | NaN%       |         |
| Generate Table Traits | 623 ms, 4 µs and 392 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 338 ms, 399 µs and 959 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 70 µs and 31 ns    | NaN%       |         |
| Generate Deletable Traits  | 35 ms, 539 µs and 351 ns  | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 570 µs and 432 ns  | NaN%       |         |
| Generate Foreign Traits    | 147 ms, 805 µs and 228 ns | NaN%       |         |
| Generate Insertable Traits | 338 ms, 399 µs and 959 ns | NaN%       |         |
| Generate Updatable Traits  | 49 ms, 619 µs and 391 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 174 ms, 777 µs and 289 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 232 µs and 216 ns  | NaN%       |         |
| procedure template Impl Codegen | 174 ms, 777 µs and 289 ns | NaN%       |         |

![Plot](time_requirements_report.png)
