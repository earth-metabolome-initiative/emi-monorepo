# Time Report for Building Core Structures

The total time spent on all tasks was 16 seconds.
The slowest task was `Code Generation` which took 7 seconds, 164 ms, 751 µs and 683 ns (43.75% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 4 seconds, 776 ms, 472 µs and 669 ns | 25.00%     |         |
| Init DB                                   | 4 seconds, 379 ms, 392 µs and 99 ns  | 25.00%     |         |
| Code Generation                           | 7 seconds, 164 ms, 751 µs and 683 ns | 43.75%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 379 ms, 279 µs and 679 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 112 µs and 420 ns                    | 0.00%      |         |
| Init DB Transaction | 4 seconds, 379 ms, 279 µs and 679 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 905 ms, 630 µs and 142 ns (75.00% of all time).

| name                  | time                                 | percentage | comment |
|-----------------------|--------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 905 ms, 630 µs and 142 ns | 75.00%     |         |
| Initialize Migrations | 473 ms, 649 µs and 537 ns            | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 996 ms, 220 µs and 878 ns (85.71% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 996 ms, 220 µs and 878 ns | 85.71%     |         |
| Procedure Codegen | 168 ms, 530 µs and 805 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 6 seconds, 165 ms, 135 µs and 185 ns (100.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 13 ms, 708 µs and 51 ns              | 0.00%      |         |
| Creating table extension network | 26 ms, 92 µs and 886 ns              | 0.00%      |         |
| Generating Diesel code           | 171 ms, 930 µs and 501 ns            | 0.00%      |         |
| Generate Structs                 | 6 seconds, 165 ms, 135 µs and 185 ns | 100.00%    |         |
| Generate Web Common Traits       | 619 ms, 354 µs and 255 ns            | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 100 ms, 919 µs and 511 ns (NaN% of all time).

| name                                                   | time                      | percentage | comment |
|--------------------------------------------------------|---------------------------|------------|---------|
| Generating tables schema                               | 67 ms, 69 µs and 84 ns    | NaN%       |         |
| Generating types schema                                | 3 ms, 941 µs and 906 ns   | NaN%       |         |
| Generating allow tables to appear in same query schema | 100 ms, 919 µs and 511 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 6 seconds, 161 ms, 751 µs and 74 ns (100.00% of all time).

| name                   | time                                | percentage | comment |
|------------------------|-------------------------------------|------------|---------|
| Generate Types Structs | 3 ms, 384 µs and 111 ns             | 0.00%      |         |
| Generate Table Structs | 6 seconds, 161 ms, 751 µs and 74 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 616 ms, 982 µs and 119 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 372 µs and 136 ns   | NaN%       |         |
| Generate Table Traits | 616 ms, 982 µs and 119 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 331 ms, 40 µs and 463 ns (NaN% of all time).

| name                       | time                     | percentage | comment |
|----------------------------|--------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 279 µs and 732 ns | NaN%       |         |
| Generate Deletable Traits  | 55 ms, 409 µs and 759 ns | NaN%       |         |
| Generate Upsertable Traits | 34 ms, 749 µs and 18 ns  | NaN%       |         |
| Generate Foreign Traits    | 133 ms, 830 µs and 91 ns | NaN%       |         |
| Generate Insertable Traits | 331 ms, 40 µs and 463 ns | NaN%       |         |
| Generate Updatable Traits  | 48 ms, 673 µs and 56 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 157 ms, 330 µs and 726 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 11 ms, 200 µs and 79 ns   | NaN%       |         |
| procedure template Impl Codegen | 157 ms, 330 µs and 726 ns | NaN%       |         |

![Plot](time_requirements_report.png)
