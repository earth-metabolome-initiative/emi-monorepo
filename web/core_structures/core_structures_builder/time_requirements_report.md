# Time Report for Building Core Structures

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB` which took 5 minutes, 26 seconds, 69 ms, 133 µs and 741 ns (94.49% of all time).

| name                                      | time                                            | percentage | comment |
|-------------------------------------------|-------------------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 3 seconds, 161 ms, 989 µs and 815 ns            | 0.87%      |         |
| Init DB                                   | 5 minutes, 26 seconds, 69 ms, 133 µs and 741 ns | 94.49%     |         |
| Code Generation                           | 16 seconds, 372 ms, 909 µs and 308 ns           | 4.64%      |         |

## Time Report for Init DB

The total time spent on all tasks was 5 minutes.
The slowest task was `Init DB Transaction` which took 5 minutes, 26 seconds, 69 ms and 748 ns (100.00% of all time).

| name                | time                                    | percentage | comment |
|---------------------|-----------------------------------------|------------|---------|
| Retrieve CSVs       | 132 µs and 993 ns                       | 0.00%      |         |
| Init DB Transaction | 5 minutes, 26 seconds, 69 ms and 748 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was 5 minutes.
The slowest task was `Consistency Constraint Checks` which took 5 minutes, 19 seconds, 301 ms, 753 µs and 45 ns (97.85% of all time).

| name                          | time                                            | percentage | comment |
|-------------------------------|-------------------------------------------------|------------|---------|
| Initialize CSVs               | 6 seconds, 78 ms, 171 µs and 114 ns             | 1.84%      |         |
| Initialize Migrations         | 689 ms, 76 µs and 589 ns                        | 0.00%      |         |
| Consistency Constraint Checks | 5 minutes, 19 seconds, 301 ms, 753 µs and 45 ns | 97.85%     |         |

#### Time Report for Consistency Constraint Checks

The total time spent on all tasks was 5 minutes.
The slowest task was `Procedure and procedure template alignment` which took 5 minutes, 10 seconds, 869 ms, 788 µs and 740 ns (97.18% of all time).

| name                                               | time                                             | percentage | comment |
|----------------------------------------------------|--------------------------------------------------|------------|---------|
| Procedure and procedure template alignment         | 5 minutes, 10 seconds, 869 ms, 788 µs and 740 ns | 97.18%     |         |
| Check constraints in schema 'public'               | 7 seconds, 873 ms, 528 µs and 974 ns             | 2.19%      |         |
| Procedure and procedure template check constraints | 558 ms, 435 µs and 331 ns                        | 0.00%      |         |

##### Time Report for Check constraints in schema 'public'

The total time spent on all tasks was now.
The slowest task was `Compatible foreign type constraints` which took 7 seconds, 774 ms, 957 µs and 446 ns (100.00% of all time).

| name                                     | time                                 | percentage | comment |
|------------------------------------------|--------------------------------------|------------|---------|
| Compatible foreign type constraints      | 7 seconds, 774 ms, 957 µs and 446 ns | 100.00%    |         |
| Lowercase column and table names         | 82 ms, 565 µs and 11 ns              | 0.00%      |         |
| Standard column names and types          | 8 ms, 959 µs and 975 ns              | 0.00%      |         |
| Not-null constraints on standard columns | 5 ms, 595 µs and 768 ns              | 0.00%      |         |
| Word deprecation constraints             | 1 ms, 450 µs and 774 ns              | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was 16 seconds.
The slowest task was `Code generation` which took 15 seconds, 540 ms, 110 µs and 806 ns (93.75% of all time).

| name              | time                                  | percentage | comment |
|-------------------|---------------------------------------|------------|---------|
| Code generation   | 15 seconds, 540 ms, 110 µs and 806 ns | 93.75%     |         |
| Procedure Codegen | 832 ms, 798 µs and 502 ns             | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was 15 seconds.
The slowest task was `Generate Structs` which took 14 seconds, 605 ms, 689 µs and 217 ns (93.33% of all time).

| name                             | time                                  | percentage | comment |
|----------------------------------|---------------------------------------|------------|---------|
| Retrieving tables                | 272 µs and 316 ns                     | 0.00%      |         |
| Creating table extension network | 657 µs and 985 ns                     | 0.00%      |         |
| Generating Diesel code           | 47 ms, 818 µs and 675 ns              | 0.00%      |         |
| Generate Structs                 | 14 seconds, 605 ms, 689 µs and 217 ns | 93.33%     |         |
| Generate Web Common Traits       | 885 ms, 672 µs and 613 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating tables schema` which took 22 ms, 424 µs and 703 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 22 ms, 424 µs and 703 ns | NaN%       |         |
| Generating types schema                                | 4 ms, 663 µs and 997 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 20 ms, 729 µs and 975 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was 14 seconds.
The slowest task was `Generate Table Structs` which took 14 seconds, 601 ms, 932 µs and 720 ns (100.00% of all time).

| name                   | time                                  | percentage | comment |
|------------------------|---------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 756 µs and 497 ns               | 0.00%      |         |
| Generate Table Structs | 14 seconds, 601 ms, 932 µs and 720 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 881 ms, 167 µs and 820 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 4 ms, 504 µs and 793 ns   | NaN%       |         |
| Generate Table Traits | 881 ms, 167 µs and 820 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 513 ms, 599 µs and 693 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 21 ms, 144 µs and 914 ns  | NaN%       |         |
| Generate Deletable Traits  | 44 ms, 168 µs and 351 ns  | NaN%       |         |
| Generate Upsertable Traits | 50 ms, 761 µs and 453 ns  | NaN%       |         |
| Generate Foreign Traits    | 203 ms, 498 µs and 340 ns | NaN%       |         |
| Generate Insertable Traits | 513 ms, 599 µs and 693 ns | NaN%       |         |
| Generate Updatable Traits  | 47 ms, 995 µs and 69 ns   | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 785 ms, 587 µs and 901 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 47 ms, 210 µs and 601 ns  | NaN%       |         |
| procedure template Impl Codegen | 785 ms, 587 µs and 901 ns | NaN%       |         |

![Plot](time_requirements_report.png)
