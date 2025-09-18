# Time Report for Building Core Structures

The total time spent on all tasks was 22 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 9 seconds, 868 ms, 62 µs and 301 ns (40.91% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 9 seconds, 868 ms, 62 µs and 301 ns  | 40.91%     |         |
| Init DB                                   | 5 seconds, 331 ms, 969 µs and 781 ns | 22.73%     |         |
| Code Generation                           | 7 seconds, 782 ms, 229 µs and 715 ns | 31.82%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 5 seconds, 331 ms, 863 µs and 931 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 105 µs and 850 ns                    | 0.00%      |         |
| Init DB Transaction | 5 seconds, 331 ms, 863 µs and 931 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 619 ms, 988 µs and 212 ns (60.00% of all time).

| name                          | time                                 | percentage | comment |
|-------------------------------|--------------------------------------|------------|---------|
| Initialize CSVs               | 3 seconds, 619 ms, 988 µs and 212 ns | 60.00%     |         |
| Initialize Migrations         | 446 ms, 34 µs and 560 ns             | 0.00%      |         |
| Consistency Constraint Checks | 1 second, 265 ms, 841 µs and 159 ns  | 20.00%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was now.
The slowest task was `Procedure and procedure template alignment` which took 736 ms, 120 µs and 664 ns (0.00% of all time).

| name                                               | time                      | percentage | comment |
|----------------------------------------------------|---------------------------|------------|---------|
| Procedure and procedure template alignment         | 736 ms, 120 µs and 664 ns | 0.00%      |         |
| Check constraints in schema 'public'               | 146 ms, 131 µs and 481 ns | 0.00%      |         |
| Procedure and procedure template check constraints | 383 ms, 589 µs and 14 ns  | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 135 ms, 56 µs and 532 ns (NaN% of all time).

| name                                     | time                     | percentage | comment |
|------------------------------------------|--------------------------|------------|---------|
| Compatible foreign type constraints      | 135 ms, 56 µs and 532 ns | NaN%       |         |
| Lowercase column and table names         | 1 ms, 161 µs and 409 ns  | NaN%       |         |
| Standard column names and types          | 5 ms, 546 µs and 243 ns  | NaN%       |         |
| Not-null constraints on standard columns | 3 ms, 423 µs and 86 ns   | NaN%       |         |
| Word deprecation constraints             | 944 µs and 211 ns        | NaN%       |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 7 seconds, 361 ms, 742 µs and 451 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 7 seconds, 361 ms, 742 µs and 451 ns | 100.00%    |         |
| Procedure Codegen | 420 ms, 487 µs and 264 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 621 ms, 522 µs and 274 ns (85.71% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 296 µs and 69 ns                     | 0.00%      |         |
| Creating table extension network | 25 ms, 820 µs and 489 ns             | 0.00%      |         |
| Generating Diesel code           | 30 ms, 168 µs and 586 ns             | 0.00%      |         |
| Generate Structs                 | 6 seconds, 621 ms, 522 µs and 274 ns | 85.71%     |         |
| Generate Web Common Traits       | 683 ms, 935 µs and 33 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 14 ms, 597 µs and 324 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 9 ms, 337 µs and 227 ns  | NaN%       |         |
| Generating types schema                                | 6 ms, 234 µs and 35 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 14 ms, 597 µs and 324 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 618 ms, 93 µs and 48 ns (100.00% of all time).

| name                   | time                               | percentage | comment |
|------------------------|------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 429 µs and 226 ns            | 0.00%      |         |
| Generate Table Structs | 6 seconds, 618 ms, 93 µs and 48 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 680 ms, 699 µs and 60 ns (NaN% of all time).

| name                  | time                     | percentage | comment |
|-----------------------|--------------------------|------------|---------|
| Generate Types Traits | 3 ms, 235 µs and 973 ns  | NaN%       |         |
| Generate Table Traits | 680 ms, 699 µs and 60 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 407 ms, 162 µs and 928 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 16 ms, 966 µs and 945 ns  | NaN%       |         |
| Generate Deletable Traits  | 33 ms, 354 µs and 192 ns  | NaN%       |         |
| Generate Upsertable Traits | 36 ms, 418 µs and 94 ns   | NaN%       |         |
| Generate Foreign Traits    | 150 ms, 47 µs and 184 ns  | NaN%       |         |
| Generate Insertable Traits | 407 ms, 162 µs and 928 ns | NaN%       |         |
| Generate Updatable Traits  | 36 ms, 749 µs and 717 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 384 ms, 573 µs and 14 ns (NaN% of all time).

| name                            | time                     | percentage | comment |
|---------------------------------|--------------------------|------------|---------|
| Procedure Impl Codegen          | 35 ms, 914 µs and 250 ns | NaN%       |         |
| procedure template Impl Codegen | 384 ms, 573 µs and 14 ns | NaN%       |         |

![Plot](time_requirements_report.png)
