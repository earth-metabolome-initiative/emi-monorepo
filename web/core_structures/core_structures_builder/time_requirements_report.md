# Time Report for Building Core Structures

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 3 seconds, 661 ms, 738 µs and 968 ns (90.15% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 597 ms, 552 µs and 602 ns            | 4.43%      |         |
| Init DB                                   | 3 minutes, 3 seconds, 661 ms, 738 µs and 968 ns | 90.15%     |         |
| Code Generation                           | 10 seconds, 303 ms, 890 µs and 371 ns           | 4.93%      |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 3 seconds, 661 ms, 624 µs and 264 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 114 µs and 704 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 3 seconds, 661 ms, 624 µs and 264 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 59 seconds, 487 ms, 328 µs and 46 ns (97.81% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 706 ms, 332 µs and 620 ns            | 1.64%      |         |
| Initialize Migrations         | 467 ms, 963 µs and 598 ns                       | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 59 seconds, 487 ms, 328 µs and 46 ns | 97.81%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 54 seconds, 249 ms, 373 µs and 427 ns (97.21% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 54 seconds, 249 ms, 373 µs and 427 ns | 97.21%     |         |
| Check constraints in schema 'public'               | 4 seconds, 891 ms, 999 µs and 155 ns             | 2.23%      |         |
| Procedure and procedure template check constraints | 345 ms, 955 µs and 464 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 835 ms, 505 µs and 379 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 835 ms, 505 µs and 379 ns | 100.00%    |         |
| Lowercase column and table names         | 46 ms, 191 µs and 438 ns             | 0.00%      |         |
| Standard column names and types          | 5 ms, 621 µs and 597 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 3 ms, 695 µs and 819 ns              | 0.00%      |         |
| Word deprecation constraints             | 984 µs and 922 ns                    | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 10 seconds, 129 ms, 6 µs and 868 ns (100.00% of all time).

| name              | time                                | percentage | comment |
|-------------------|-------------------------------------|------------|---------|
| Code generation   | 10 seconds, 129 ms, 6 µs and 868 ns | 100.00%    |         |
| Procedure Codegen | 174 ms, 883 µs and 503 ns           | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 9 seconds, 525 ms, 836 µs and 240 ns (90.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 116 µs and 86 ns                     | 0.00%      |         |
| Creating table extension network | 368 µs and 158 ns                    | 0.00%      |         |
| Generating Diesel code           | 30 ms, 8 µs and 822 ns               | 0.00%      |         |
| Generate Structs                 | 9 seconds, 525 ms, 836 µs and 240 ns | 90.00%     |         |
| Generate Web Common Traits       | 572 ms, 677 µs and 562 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 13 ms, 962 µs and 583 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 13 ms, 962 µs and 583 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 381 µs and 753 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 12 ms, 664 µs and 486 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 9 seconds, 523 ms, 232 µs and 55 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 604 µs and 185 ns             | 0.00%      |         |
| Generate Table Structs | 9 seconds, 523 ms, 232 µs and 55 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 570 ms, 50 µs and 342 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 2 ms, 627 µs and 220 ns  | NaN%       |         |
| Generate Table Traits | 570 ms, 50 µs and 342 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 333 ms, 141 µs and 861 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 215 µs and 201 ns  | NaN%       |         |
| Generate Deletable Traits  | 28 ms, 951 µs and 631 ns  | NaN%       |         |
| Generate Upsertable Traits | 32 ms, 606 µs and 889 ns  | NaN%       |         |
| Generate Foreign Traits    | 132 ms, 715 µs and 981 ns | NaN%       |         |
| Generate Insertable Traits | 333 ms, 141 µs and 861 ns | NaN%       |         |
| Generate Updatable Traits  | 29 ms, 418 µs and 779 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 163 ms, 439 µs and 316 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 444 µs and 187 ns  | NaN%       |         |
| procedure template Impl Codegen | 163 ms, 439 µs and 316 ns | NaN%       |         |

![Plot](time_requirements_report.png)
