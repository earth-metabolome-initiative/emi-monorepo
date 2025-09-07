# Time Report for Building Core Structures

The total time spent on all tasks was 18 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 8 seconds, 67 ms, 727 µs and 180 ns (44.44% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 8 seconds, 67 ms, 727 µs and 180 ns  | 44.44%     |         |
| Init DB                                   | 4 seconds, 285 ms, 636 µs and 570 ns | 22.22%     |         |
| Code Generation                           | 6 seconds, 152 ms, 417 µs and 750 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 285 ms, 495 µs and 166 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 141 µs and 404 ns                    | 0.00%      |         |
| Init DB Transaction | 4 seconds, 285 ms, 495 µs and 166 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 812 ms, 440 µs and 452 ns (75.00% of all time).

| name                  | time                                 | percentage | comment |
|-----------------------|--------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 812 ms, 440 µs and 452 ns | 75.00%     |         |
| Initialize Migrations | 473 ms, 54 µs and 714 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 5 seconds, 995 ms, 187 µs and 872 ns (83.33% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 5 seconds, 995 ms, 187 µs and 872 ns | 83.33%     |         |
| Procedure Codegen | 157 ms, 229 µs and 878 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 136 ms, 317 µs and 202 ns (100.00% of all time).

| name                             | time                                 | percentage | comment |
|----------------------------------|--------------------------------------|------------|---------|
| Retrieving tables                | 11 ms, 176 µs and 734 ns             | 0.00%      |         |
| Creating table extension network | 16 ms, 32 µs and 681 ns              | 0.00%      |         |
| Generating Diesel code           | 155 ms, 644 µs and 447 ns            | 0.00%      |         |
| Generate Structs                 | 5 seconds, 136 ms, 317 µs and 202 ns | 100.00%    |         |
| Generate Web Common Traits       | 676 ms, 16 µs and 808 ns             | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 84 ms, 53 µs and 840 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 67 ms, 633 µs and 491 ns | NaN%       |         |
| Generating types schema                                | 3 ms, 957 µs and 116 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 84 ms, 53 µs and 840 ns  | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 133 ms, 483 µs and 167 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 834 µs and 35 ns               | 0.00%      |         |
| Generate Table Structs | 5 seconds, 133 ms, 483 µs and 167 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 673 ms, 763 µs and 584 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 253 µs and 224 ns   | NaN%       |         |
| Generate Table Traits | 673 ms, 763 µs and 584 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 387 ms, 903 µs and 10 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 14 ms, 599 µs and 169 ns  | NaN%       |         |
| Generate Deletable Traits  | 51 ms, 566 µs and 798 ns  | NaN%       |         |
| Generate Upsertable Traits | 33 ms, 204 µs and 347 ns  | NaN%       |         |
| Generate Foreign Traits    | 131 ms, 734 µs and 951 ns | NaN%       |         |
| Generate Insertable Traits | 387 ms, 903 µs and 10 ns  | NaN%       |         |
| Generate Updatable Traits  | 54 ms, 755 µs and 309 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 146 ms, 916 µs and 713 ns (NaN% of all time).

| name                            | time                      | percentage | comment |
|---------------------------------|---------------------------|------------|---------|
| Procedure Impl Codegen          | 10 ms, 313 µs and 165 ns  | NaN%       |         |
| procedure template Impl Codegen | 146 ms, 916 µs and 713 ns | NaN%       |         |

![Plot](time_requirements_report.png)
