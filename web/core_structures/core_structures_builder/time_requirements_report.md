# Time Report for Building Core Structures

The total time spent on all tasks was 22 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 377 ms, 654 µs and 736 ns (40.91% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 377 ms, 654 µs and 736 ns | 40.91%     |         |
| Init DB                                   | 5 seconds, 475 ms, 817 µs and 171 ns | 22.73%     |         |
| Code Generation                           | 7 seconds, 505 ms, 950 µs and 173 ns | 31.82%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 475 ms, 682 µs and 808 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 134 µs and 363 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 475 ms, 682 µs and 808 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 678 ms, 929 µs and 409 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 678 ms, 929 µs and 409 ns | 60.00%     |         |
| Initialize Migrations         | 450 ms, 127 µs and 626 ns            | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 346 ms, 625 µs and 773 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 799 ms, 997 µs and 602 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 799 ms, 997 µs and 602 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 156 ms, 360 µs and 529 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 390 ms, 267 µs and 642 ns | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 143 ms, 581 µs and 379 ns (NaN% of all time).

| name                                     | time                      | percentage | comment |
|------------------------------------------|---------------------------|------------|---------|
| Compatible foreign type constraints      | 143 ms, 581 µs and 379 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 177 µs and 514 ns   | NaN%       |         |
| Standard column names and types          | 6 ms, 208 µs and 306 ns   | NaN%       |         |
| Not-null constraints on standard columns | 4 ms, 11 µs and 567 ns    | NaN%       |         |
| Word deprecation constraints             | 1 ms, 381 µs and 763 ns   | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 121 ms, 208 µs and 865 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 121 ms, 208 µs and 865 ns | 100.00%    |         |
| Procedure Codegen | 384 ms, 741 µs and 308 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 444 ms, 789 µs and 658 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 268 µs and 657 ns                    | 0.00%      |         |
| Creating table extension network | 15 ms, 630 µs and 492 ns             | 0.00%      |         |
| Generating Diesel code           | 27 ms, 457 µs and 440 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 444 ms, 789 µs and 658 ns | 85.71%     |         |
| Generate Web Common Traits       | 633 ms, 62 µs and 618 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 13 ms, 498 µs and 722 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 712 µs and 265 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 246 µs and 453 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 13 ms, 498 µs and 722 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 442 ms, 150 µs and 649 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 639 µs and 9 ns                | 0.00%      |         |
| Generate Table Structs | 6 seconds, 442 ms, 150 µs and 649 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 630 ms, 429 µs and 769 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 632 µs and 849 ns   | NaN%       |         |
| Generate Table Traits | 630 ms, 429 µs and 769 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 372 ms, 822 µs and 760 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 485 µs and 823 ns  | NaN%       |         |
| Generate Deletable Traits  | 31 ms, 211 µs and 758 ns  | NaN%       |         |
| Generate Upsertable Traits | 32 ms, 958 µs and 95 ns   | NaN%       |         |
| Generate Foreign Traits    | 147 ms, 906 µs and 31 ns  | NaN%       |         |
| Generate Insertable Traits | 372 ms, 822 µs and 760 ns | NaN%       |         |
| Generate Updatable Traits  | 32 ms, 45 µs and 302 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 349 ms, 471 µs and 952 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 35 ms, 269 µs and 356 ns  | NaN%       |         |
| procedure template Impl Codegen | 349 ms, 471 µs and 952 ns | NaN%       |         |

![Plot](time_requirements_report.png)
