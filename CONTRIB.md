# How to contribute

## Setting up your Rust environment

I wrote a short [Rust setup tutorial](https://github.com/LucaCappelletti94/linux-setup/blob/main/RUST.md) that you can follow to set up your Rust environment. Please follow it before contributing to this project.

## Setting up the Monorepo environment

In the monorepo you will find 

```bash
pip install pre-commit
```

## Comments on code styles and standards

In this section I try to briefly summarize the code styles and standards that we follow in this project. If you are unsure about something, please refer to the existing codebase, or ask in the GitHub issues.

### On code generation

As in any project, and expecially any project involving web development, there is a considerable amount of boiler plate code which needs to remain self-consistent. Handling this boiler plate code manually is error-prone and time-consuming, and leads to significantly more cognitive load when working on the project. Since we are a small team, we cannot afford to waste time on such tasks, and we automate them by using code generation whenever possible.

There are two genres of code generation: either simple [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html), as happens when you simply have a trait that you can often implement automatically, or more complex code generation, as happens when you have a pattern that is repeated in the codebase like in the case of code inferred from the PostgreSQL schema, for instance methods that generate queries for foreign keys of a table, or methods which use available table indices to execute search queries.

The second kind of code generation, both for Rust and SQL, is primarily handled in the [`webcodegen`](`web/web_common/webcodegen`) crate. Whenever you identify a pattern that is repeated in the codebase, please consider whether it can be automated. If you are unsure, please open an issue on the GitHub repository and we will discuss it together.

### Dependency centralization

We try to keep the dependencies centralized in the [`Cargo.toml`](Cargo.toml) file of the root crate. This is to ensure that we can easily update dependencies and keep them in sync. If you need to add a new dependency, you most often will want to add it to the root crate. If you are unsure, please open an issue on the GitHub repository and we will discuss it together. In subcrates, you will specify the dependency as a feature of the root crate using:

```toml
[dependencies]
crate_name.workspace = true
# And if the dependency is optional
another_crate { workspace = true, optional = true }
```

### Performance analysis

One of the best tools available for a rough primary performance analysis is the `perf` tool. You can install it by running:

```bash
sudo perf top
```

## Contributing to the backend

The main backend code is found in [`web/backend`](web/backend). You will find therein another [`CONTRIB.md`](web/backend/CONTRIB.md) file with instructions on how to contribute to the backend.

## Contributing to the frontend

The main frontend code is found in [`web/frontend`](web/frontend). You will find therein another [`CONTRIB.md`](web/frontend/CONTRIB.md) file with instructions on how to contribute to the frontend.
