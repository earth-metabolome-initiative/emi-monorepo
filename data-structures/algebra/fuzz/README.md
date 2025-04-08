# Fuzzing harnesses for algebraic data structures

This crate provides harnesses to run fuzzing on algorithms implemented for structs in the `algebra` crate.

## Setup

Install the dependency for [honggfuzz](https://docs.rs/honggfuzz/latest/honggfuzz/) for a linux system:

```bash
sudo apt install build-essential binutils-dev libunwind-dev
```

Next, install [honggfuzz](https://docs.rs/honggfuzz/latest/honggfuzz/) itself:

```bash
cargo install honggfuzz
```

## Available harnesses

### CSR2D

The CSR2D struct is one of the most commonly used sparse matrix representations. As such, it is worth fuzzing extensively.

```bash
cargo hfuzz run csr2d
```

and to run the crash cases:

```bash
cargo hfuzz run-debug csr2d hfuzz_workspace/*/*.fuzz
```

### Valued CSR2D

The Valued CSR2D struct is a variant of the CSR2D struct that allows for storing values in the sparse matrix. This is useful for applications where the values of the non-zero elements are important.

```bash
cargo hfuzz run valued_csr2d
```

and to run the crash cases:

```bash
cargo hfuzz run-debug valued_csr2d hfuzz_workspace/*/*.fuzz
```

### Padded Matrix2d

The Padded Matrix2d struct is wrapper struct which fills in all missing values in the underlying matrix with a provided lambda function.

```bash
cargo hfuzz run padded_matrix2d
```

and to run the crash cases:

```bash
cargo hfuzz run-debug padded_matrix2d hfuzz_workspace/*/*.fuzz
```

### Generic Matrix2D with padded diagonal

The `GenericMatrix2DWithPaddedDiagonal` struct is a generic matrix representation that allows implicitly squaring and padding the diagonal. This is useful for applications where the underlying matrix is not squared or may not have a fully populated diagonal, but some algorithms require a square matrix with a fully populated diagonal.

```bash
cargo hfuzz run generic_matrix2d_with_padded_diagonal
```

and to run the crash cases:

```bash
cargo hfuzz run-debug generic_matrix2d_with_padded_diagonal hfuzz_workspace/*/*.fuzz
```

### Hopcroft-Karp

The Hopcroft-Karp algorithm is a combinatorial algorithm for finding maximum cardinality matchings in bipartite graphs.
It is implemented for all structs implementing `SparseMatrix2D`.

```bash
cargo hfuzz run hopcroft_karp
```

### LAPJV

The LAPJV algorithm is a combinatorial algorithm for finding a weighted maximum cardinality matchings in bipartite graphs.
It is implemented for all structs implementing `SparseValuedMatrix2D` and `DenseValuedMatrix2D`, respectively in the
sparse and dense cases.

```bash
cargo hfuzz run lapjv
```

and to run the crash cases:

```bash
cargo hfuzz run-debug lapjv hfuzz_workspace/*/*.fuzz
```
