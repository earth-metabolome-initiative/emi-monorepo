# Time Report for Building Core Structures

The total time spent on all tasks was 18 seconds.
The slowest task was `Setting up Docker and Database Connection` which took 7 seconds, 732 ms, 333 µs and 270 ns (38.89% of all time).

| name                                      | time                                 | percentage | comment |
|-------------------------------------------|--------------------------------------|------------|---------|
| Setting up Docker and Database Connection | 7 seconds, 732 ms, 333 µs and 270 ns | 38.89%     |         |
| Init DB                                   | 4 seconds, 302 ms, 433 µs and 682 ns | 22.22%     |         |
| Code Generation                           | 6 seconds, 551 ms, 554 µs and 259 ns | 33.33%     |         |

## Time Report for Init DB

The total time spent on all tasks was now.
The slowest task was `Init DB Transaction` which took 4 seconds, 302 ms, 299 µs and 919 ns (100.00% of all time).

| name                | time                                 | percentage | comment |
|---------------------|--------------------------------------|------------|---------|
| Retrieve CSVs       | 133 µs and 763 ns                    | 0.00%      |         |
| Init DB Transaction | 4 seconds, 302 ms, 299 µs and 919 ns | 100.00%    |         |

### Time Report for Init DB Transaction

The total time spent on all tasks was now.
The slowest task was `Initialize CSVs` which took 3 seconds, 831 ms, 217 µs and 102 ns (75.00% of all time).

| name                  | time                                 | percentage | comment |
|-----------------------|--------------------------------------|------------|---------|
| Initialize CSVs       | 3 seconds, 831 ms, 217 µs and 102 ns | 75.00%     |         |
| Initialize Migrations | 471 ms, 82 µs and 817 ns             | 0.00%      |         |

## Time Report for Code Generation

The total time spent on all tasks was now.
The slowest task was `Code generation` which took 6 seconds, 397 ms, 242 µs and 983 ns (100.00% of all time).

| name              | time                                 | percentage | comment |
|-------------------|--------------------------------------|------------|---------|
| Code generation   | 6 seconds, 397 ms, 242 µs and 983 ns | 100.00%    |         |
| Procedure Codegen | 154 ms, 311 µs and 276 ns            | 0.00%      |         |

### Time Report for Code generation

The total time spent on all tasks was now.
The slowest task was `Generate Structs` which took 5 seconds, 607 ms, 41 µs and 161 ns (83.33% of all time).

| name                             | time                                | percentage | comment |
|----------------------------------|-------------------------------------|------------|---------|
| Retrieving tables                | 10 ms, 539 µs and 178 ns            | 0.00%      |         |
| Creating table extension network | 16 ms, 912 µs and 104 ns            | 0.00%      |         |
| Generating Diesel code           | 128 ms, 946 µs and 285 ns           | 0.00%      |         |
| Generate Structs                 | 5 seconds, 607 ms, 41 µs and 161 ns | 83.33%     |         |
| Generate Web Common Traits       | 633 ms, 804 µs and 255 ns           | 0.00%      |         |

#### Time Report for Generating Diesel code

The total time spent on all tasks was now.
The slowest task was `Generating allow tables to appear in same query schema` which took 82 ms, 203 µs and 131 ns (NaN% of all time).

| name                                                   | time                     | percentage | comment |
|--------------------------------------------------------|--------------------------|------------|---------|
| Generating tables schema                               | 42 ms, 96 µs and 754 ns  | NaN%       |         |
| Generating types schema                                | 4 ms, 646 µs and 400 ns  | NaN%       |         |
| Generating allow tables to appear in same query schema | 82 ms, 203 µs and 131 ns | NaN%       |         |

#### Time Report for Generate Structs

The total time spent on all tasks was now.
The slowest task was `Generate Table Structs` which took 5 seconds, 604 ms, 354 µs and 430 ns (100.00% of all time).

| name                   | time                                 | percentage | comment |
|------------------------|--------------------------------------|------------|---------|
| Generate Types Structs | 2 ms, 686 µs and 731 ns              | 0.00%      |         |
| Generate Table Structs | 5 seconds, 604 ms, 354 µs and 430 ns | 100.00%    |         |

#### Time Report for Generate Web Common Traits

The total time spent on all tasks was now.
The slowest task was `Generate Table Traits` which took 631 ms, 225 µs and 398 ns (NaN% of all time).

| name                  | time                      | percentage | comment |
|-----------------------|---------------------------|------------|---------|
| Generate Types Traits | 2 ms, 578 µs and 857 ns   | NaN%       |         |
| Generate Table Traits | 631 ms, 225 µs and 398 ns | NaN%       |         |

##### Time Report for Generate Table Traits

The total time spent on all tasks was now.
The slowest task was `Generate Insertable Traits` which took 354 ms, 615 µs and 510 ns (NaN% of all time).

| name                       | time                      | percentage | comment |
|----------------------------|---------------------------|------------|---------|
| Generate CRUD Traits       | 13 ms, 124 µs and 95 ns   | NaN%       |         |
| Generate Deletable Traits  | 54 ms, 330 µs and 309 ns  | NaN%       |         |
| Generate Upsertable Traits | 32 ms, 984 µs and 627 ns  | NaN%       |         |
| Generate Foreign Traits    | 130 ms, 317 µs and 432 ns | NaN%       |         |
| Generate Insertable Traits | 354 ms, 615 µs and 510 ns | NaN%       |         |
| Generate Updatable Traits  | 45 ms, 853 µs and 425 ns  | NaN%       |         |

### Time Report for Procedure Codegen

The total time spent on all tasks was now.
The slowest task was `procedure template Impl Codegen` which took 144 ms, 58 µs and 471 ns (NaN% of all time).

| name                            | time                     | percentage | comment |
|---------------------------------|--------------------------|------------|---------|
| Procedure Impl Codegen          | 10 ms, 252 µs and 805 ns | NaN%       |         |
| procedure template Impl Codegen | 144 ms, 58 µs and 471 ns | NaN%       |         |

![Plot](time_requirements_report.png)
