# Time Report for Building Core Structures

The total time spent on all tasks was 23 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 603 ms, 333 µs and 657 ns (39.13% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 603 ms, 333 µs and 657 ns | 39.13%     |         |
| Init DB                                   | 5 seconds, 763 ms, 584 µs and 538 ns | 21.74%     |         |
| Code Generation                           | 7 seconds, 767 ms, 813 µs and 430 ns | 30.43%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 763 ms, 459 µs and 209 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 125 µs and 329 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 763 ms, 459 µs and 209 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 873 ms, 315 µs and 656 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 873 ms, 315 µs and 656 ns | 60.00%     |         |
| Initialize Migrations         | 475 ms, 616 µs and 11 ns             | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 414 ms, 527 µs and 542 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 824 ms, 853 µs and 145 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 824 ms, 853 µs and 145 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 164 ms, 309 µs and 552 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 425 ms, 364 µs and 845 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 151 ms, 804 µs and 280 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 151 ms, 804 µs and 280 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 485 µs and 290 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 22 µs and 292 ns    | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 749 µs and 399 ns   | NaN%       |         |
| Word deprecation constraints             | 1 ms, 248 µs and 291 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 576 ms, 232 µs and 531 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 576 ms, 232 µs and 531 ns | 100.00%    |         |
| Procedure Codegen | 191 ms, 580 µs and 899 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 885 ms, 355 µs and 962 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 312 µs and 624 ns                    | 0.00%      |         |
| Creating table extension network | 17 ms, 185 µs and 952 ns             | 0.00%      |         |
| Generating Diesel code           | 27 ms, 835 µs and 673 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 885 ms, 355 µs and 962 ns | 85.71%     |         |
| Generate Web Common Traits       | 645 ms, 542 µs and 320 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 127 µs and 619 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 8 ms, 804 µs and 326 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 903 µs and 728 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 127 µs and 619 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 881 ms, 547 µs and 303 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 808 µs and 659 ns              | 0.00%      |         |
| Generate Table Structs | 6 seconds, 881 ms, 547 µs and 303 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 642 ms, 515 µs and 735 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 26 µs and 585 ns    | NaN%       |         |
| Generate Table Traits | 642 ms, 515 µs and 735 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 354 ms, 581 µs and 338 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 15 ms, 431 µs and 514 ns  | NaN%       |         |
| Generate Deletable Traits  | 35 ms, 520 µs and 833 ns  | NaN%       |         |
| Generate Upsertable Traits | 35 ms, 59 µs and 435 ns   | NaN%       |         |
| Generate Foreign Traits    | 149 ms, 527 µs and 521 ns | NaN%       |         |
| Generate Insertable Traits | 354 ms, 581 µs and 338 ns | NaN%       |         |
| Generate Updatable Traits  | 52 ms, 395 µs and 94 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 179 ms, 263 µs and 322 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 12 ms, 317 µs and 577 ns  | NaN%       |         |
| procedure template Impl Codegen | 179 ms, 263 µs and 322 ns | NaN%       |         |

![Plot](time_requirements_report.png)
