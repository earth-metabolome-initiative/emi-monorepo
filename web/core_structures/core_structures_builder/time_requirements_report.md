# Time Report for Building Core Structures

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB` which took 3 minutes, 2 seconds, 689 ms, 968 µs and 292 ns (90.10% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 400 ms, 30 µs and 726 ns             | 4.46%      |         |
| Init DB                                   | 3 minutes, 2 seconds, 689 ms, 968 µs and 292 ns | 90.10%     |         |
| Code Generation                           | 9 seconds, 962 ms, 81 µs and 664 ns             | 4.46%      |         |

## Time Report for Init DB

The total time spent on all tasks was 3 minutes.
The slowest task was `Init DB Transaction` which took 3 minutes, 2 seconds, 689 ms, 858 µs and 556 ns (100.00% of all time).

| name                | time                                            | percentage | comment |
|---------------------|-------------------------------------------------|------------|---------|
| Retrieve CSVs       | 109 µs and 736 ns                               | 0.00%      |         |
| Init DB Transaction | 3 minutes, 2 seconds, 689 ms, 858 µs and 556 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 3 minutes.
The slowest task was `Consistency Constraint Checks` which took 2 minutes, 58 seconds, 544 ms, 960 µs and 82 ns (97.80% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 696 ms, 882 µs and 438 ns            | 1.65%      |         |
| Initialize Migrations         | 448 ms, 16 µs and 36 ns                         | 0.00%      |         |
| Consistency Constraint Checks | 2 minutes, 58 seconds, 544 ms, 960 µs and 82 ns | 97.80%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 2 minutes.
The slowest task was `Procedure and procedure template alignment` which took 2 minutes, 53 seconds, 356 ms, 425 µs and 868 ns (97.19% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 2 minutes, 53 seconds, 356 ms, 425 µs and 868 ns | 97.19%     |         |
| Check constraints in schema 'public'               | 4 seconds, 846 ms, 314 µs and 617 ns             | 2.25%      |         |
| Procedure and procedure template check constraints | 342 ms, 219 µs and 597 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 4 seconds, 790 ms, 8 µs and 565 ns (100.00% of all time).

| name                                     | time                               | percentage | comment |
|------------------------------------------|------------------------------------|------------|---------|
| Compatible foreign type constraints      | 4 seconds, 790 ms, 8 µs and 565 ns | 100.00%    |         |
| Lowercase column and table names         | 46 ms, 342 µs and 28 ns            | 0.00%      |         |
| Standard column names and types          | 5 ms, 560 µs and 469 ns            | 0.00%      |         |
| Not-null constraints on standard columns | 3 ms, 480 µs and 656 ns            | 0.00%      |         |
| Word deprecation constraints             | 922 µs and 899 ns                  | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 9 seconds, 792 ms, 103 µs and 885 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 9 seconds, 792 ms, 103 µs and 885 ns | 100.00%    |         |
| Procedure Codegen | 169 ms, 977 µs and 779 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 9 seconds, 205 ms, 186 µs and 835 ns (100.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 172 µs and 832 ns                    | 0.00%      |         |
| Creating table extension network | 481 µs and 950 ns                    | 0.00%      |         |
| Generating Diesel code           | 31 ms, 849 µs and 308 ns             | 0.00%      |         |
| Generate Structs                 | 9 seconds, 205 ms, 186 µs and 835 ns | 100.00%    |         |
| Generate Web Common Traits       | 554 ms, 412 µs and 960 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 14 ms, 940 µs and 547 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 14 ms, 940 µs and 547 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 670 µs and 203 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 13 ms, 238 µs and 558 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 9 seconds, 202 ms, 442 µs and 515 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 744 µs and 320 ns              | 0.00%      |         |
| Generate Table Structs | 9 seconds, 202 ms, 442 µs and 515 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 551 ms, 748 µs and 41 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 2 ms, 664 µs and 919 ns  | NaN%       |         |
| Generate Table Traits | 551 ms, 748 µs and 41 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 316 ms, 727 µs and 558 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 228 µs and 212 ns  | NaN%       |         |
| Generate Deletable Traits  | 27 ms, 857 µs and 186 ns  | NaN%       |         |
| Generate Upsertable Traits | 30 ms, 993 µs and 942 ns  | NaN%       |         |
| Generate Foreign Traits    | 132 ms, 928 µs and 198 ns | NaN%       |         |
| Generate Insertable Traits | 316 ms, 727 µs and 558 ns | NaN%       |         |
| Generate Updatable Traits  | 30 ms, 12 µs and 945 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 159 ms, 637 µs and 254 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 10 ms, 340 µs and 525 ns  | NaN%       |         |
| procedure template Impl Codegen | 159 ms, 637 µs and 254 ns | NaN%       |         |

![Plot](time_requirements_report.png)
