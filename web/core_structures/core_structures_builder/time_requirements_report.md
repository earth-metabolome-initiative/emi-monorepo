# Time Report for Building Core Structures

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB` which took 6 minutes, 32 seconds, 552 ms, 439 µs and 852 ns (94.92% of all time).

| name                                      | time                                             | percentage | comment |
|-------------------------------------------|--------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 136 ms, 70 µs and 73 ns               | 0.73%      |         |
| Init DB                                   | 6 minutes, 32 seconds, 552 ms, 439 µs and 852 ns | 94.92%     |         |
| Code Generation                           | 17 seconds, 883 ms, 771 µs and 581 ns            | 4.12%      |         |

## Time Report for Init DB

The total time spent on all tasks was 6 minutes.
The slowest task was `Init DB Transaction` which took 6 minutes, 32 seconds, 552 ms, 231 µs and 577 ns (100.00% of all time).

| name                | time                                             | percentage | comment |
|---------------------|--------------------------------------------------|------------|---------|
| Retrieve CSVs       | 208 µs and 275 ns                                | 0.00%      |         |
| Init DB Transaction | 6 minutes, 32 seconds, 552 ms, 231 µs and 577 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 6 minutes.
The slowest task was `Consistency Constraint Checks` which took 6 minutes, 25 seconds, 809 ms, 855 µs and 786 ns (98.21% of all time).

| name                          | time                                             | percentage | comment |
|-------------------------------|--------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 57 ms, 171 µs and 820 ns              | 1.53%      |         |
| Initialize Migrations         | 685 ms, 203 µs and 971 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 6 minutes, 25 seconds, 809 ms, 855 µs and 786 ns | 98.21%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 6 minutes.
The slowest task was `Procedure and procedure template alignment` which took 6 minutes, 16 seconds, 170 ms, 107 µs and 694 ns (97.66% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 6 minutes, 16 seconds, 170 ms, 107 µs and 694 ns | 97.66%     |         |
| Check constraints in schema 'public'               | 8 seconds, 531 ms, 778 µs and 218 ns             | 2.08%      |         |
| Procedure and procedure template check constraints | 1 second, 107 ms, 969 µs and 874 ns              | 0.26%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 8 seconds, 411 ms, 973 µs and 868 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 8 seconds, 411 ms, 973 µs and 868 ns | 100.00%    |         |
| Lowercase column and table names         | 85 ms, 678 µs and 381 ns             | 0.00%      |         |
| Standard column names and types          | 18 ms, 330 µs and 374 ns             | 0.00%      |         |
| Not-null constraints on standard columns | 12 ms, 273 µs and 174 ns             | 0.00%      |         |
| Word deprecation constraints             | 3 ms, 522 µs and 421 ns              | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 17 seconds.
The slowest task was `Code generation` which took 16 seconds, 842 ms, 504 µs and 72 ns (94.12% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 16 seconds, 842 ms, 504 µs and 72 ns | 94.12%     |         |
| Procedure Codegen | 1 second, 41 ms, 267 µs and 509 ns   | 5.88%      |         |

### Time Report for Code generation

The total time spent on all tasks was 16 seconds.
The slowest task was `Generate Structs` which took 15 seconds, 852 ms, 849 µs and 617 ns (93.75% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 251 µs and 565 ns                     | 0.00%      |         |
| Creating table extension network | 800 µs and 809 ns                     | 0.00%      |         |
| Generating Diesel code           | 56 ms, 407 µs and 494 ns              | 0.00%      |         |
| Generate Structs                 | 15 seconds, 852 ms, 849 µs and 617 ns | 93.75%     |         |
| Generate Web Common Traits       | 932 ms, 194 µs and 587 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 28 ms, 690 µs and 113 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 28 ms, 690 µs and 113 ns | NaN%       |         |
| Generating types schema                                | 6 ms, 617 µs and 913 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 21 ms, 99 µs and 468 ns  | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 15 seconds.
The slowest task was `Generate Table Structs` which took 15 seconds, 847 ms, 776 µs and 770 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 5 ms, 72 µs and 847 ns                | 0.00%      |         |
| Generate Table Structs | 15 seconds, 847 ms, 776 µs and 770 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 928 ms, 526 µs and 392 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 3 ms, 668 µs and 195 ns   | NaN%       |         |
| Generate Table Traits | 928 ms, 526 µs and 392 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 543 ms, 222 µs and 712 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 22 ms, 434 µs and 939 ns  | NaN%       |         |
| Generate Deletable Traits  | 44 ms, 511 µs and 149 ns  | NaN%       |         |
| Generate Upsertable Traits | 53 ms, 895 µs and 676 ns  | NaN%       |         |
| Generate Foreign Traits    | 214 ms, 644 µs and 524 ns | NaN%       |         |
| Generate Insertable Traits | 543 ms, 222 µs and 712 ns | NaN%       |         |
| Generate Updatable Traits  | 49 ms, 817 µs and 392 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 988 ms, 351 µs and 125 ns (0.00% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 52 ms, 916 µs and 384 ns  | 0.00%      |         |
| procedure template Impl Codegen | 988 ms, 351 µs and 125 ns | 0.00%      |         |

![Plot](time_requirements_report.png)
