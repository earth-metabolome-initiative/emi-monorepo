# PGRX Validation Derive

Procedural macros to dispatch the function output data type according to the features enabled.

## Validation

The primary macro made available by this crate is [`validation`]. This macro is used to generate a function that validates the input data and returns the output data, and enforces several rules we have in our codebase.

1. The function must be [public](https://doc.rust-lang.org/reference/visibility-and-privacy.html).
2. The function must not be [async](https://rust-lang.github.io/async-book/), as this is not supported by [`pgrx`](https://github.com/pgcentralfoundation/pgrx).
3. The function must not have [generic parameters](https://doc.rust-lang.org/reference/items/generics.html), as this would not be supported by [PostgreSQL](https://www.postgresql.org/).
4. The function must have at least one argument.
5. The function must return `Result<(), validation_errors::Error>`.
6. The function name must be in [snake_case](https://en.wikipedia.org/wiki/Snake_case).
7. The function name must start with `must_be_` or `must_not_be_`.
8. The function must have a doc comment.
