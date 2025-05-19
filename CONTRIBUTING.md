# How to contribute

We are happy to have you here! This is a short guide on how to contribute to the project. If you have any questions, please feel free to open an issue on the GitHub repository and we will be happy to help you.

## Why is it in Rust?

We are using [Rust](https://www.rust-lang.org/) for the Earth Metabolome Initiative (EMI) project because it is a modern language that is fast, safe, and has a great community. The EMI project goal is to catalogue the Earth Metabolome, and this will take a long time, and therefore the code base will be maintained for a long time - [no spaghetti code allowed](https://xkcd.com/1926/).

The EMI project is primarily a data collection, analysis and sharing project, and is therefore paramount to ensure data integrity and consistency. By using a strongly typed language like Rust, we can ensure that the data we are working with is **structurally** correct, meaning we cannot instantiate a struct with the wrong type, and by using these structs in the database, we can ensure that the data we are storing is necessarily correct.

We use this language for all parts of the project, including the PostgreSQL extensions (e.g. custom types in the database, [`pgrx`-based](https://github.com/pgcentralfoundation/pgrx)), the backend (e.g. the web server, [`actix`-based](https://actix.rs/)), the WASM frontend SQLite database ([`sqlite-wasm-rs`-based](https://github.com/Spxg/sqlite-wasm-rs)) and the WASM U/I frontend (e.g. the web application itself, [`yew`-based](https://github.com/yewstack/yew)) - we use the same ORM (Object-Relational Mapping) for both the backend and frontend, i.e. [`diesel`](https://github.com/diesel-rs/diesel). By using a single general language for all parts of the project, the code reusability across all parts of the project is maximal, and any new developer has only to know one language to be able to work on all parts of the project - no need to know JavaScript, TypeScript, Python, etc. Furthermore, [`Rust`](https://www.rust-lang.org/) provides many tools to ensure that the code created is correct at compile time, allowing us to catch many errors before they happen, and avoid introducing inconsistencies in the codebase: for instance, if you change the schema of a table in the database, you will be forced to change the code that uses it, and the compiler will help you to do so. This is a great advantage over other languages, where you may not notice that you have broken something until it is too late.

### Why is it in a monorepo?

A monorepo is a single repository that contains all the code for a project, including all the subprojects. The main advantage of a monorepo is that it allows us to keep all the code in one place, making it easier to manage dependencies and keep everything in sync. It also allows us to share code between different parts of the project more easily, and to ensure that all parts of the project are using the same version of a dependency. Rust makes this particularly easy, as the language comes designed with [the `workspace` feature](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html), which allows us to have multiple crates in a single repository, and to manage dependencies between them easily.

### Learning Rust

If you are new to Rust, here are a few resources which you may want to check out before starting to contribute:

- [The Rust Book](https://rust-book.cs.brown.edu/): the official Rust book, which is a great starting point for learning Rust.
- [Rustlings](https://github.com/rust-lang/rustlings): a set of small exercises to get you used to reading and writing Rust code.
- [Anki Rust Flashcards](https://github.com/ad-si/Rust-Flashcards?tab=readme-ov-file): a set of Anki flashcards to help you learn Rust. You can use the [Anki app](https://apps.ankiweb.net/) to study them.

### Setting up your Rust environment

I wrote a short [Rust setup tutorial](https://github.com/LucaCappelletti94/linux-setup/blob/main/RUST.md) that you can follow to set up your Rust environment. Please follow it before contributing to this project.

## Comments on code styles and standards

In this section I try to briefly summarize the code styles and standards that we follow in this project. If you are unsure about something, please refer to the existing codebase, or ask in the GitHub issues.

### Crate naming convention

The directories and crates should be named in `snake_case`, as it is not possible for the `pgrx` extension (which need to be installed in postgres) to have use the `kebab-case` naming convention as they cannot contain `-` characters. This is done to avoid confusion regarding the naming of the directories and crates.

### On code generation

_This section is NOT about using Copilot or other such tools in the codebase._

As in any project, and especially any project involving web development, there is a considerable amount of boiler plate code which needs to remain self-consistent. Handling this boiler plate code manually is error-prone and time-consuming, and leads to significantly more cognitive load when working on the project. Since we are a small team, we cannot afford to waste time on such tasks, and we automate them by using code generation whenever possible.

There are two genres of code generation: either simple [procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html), as happens when you simply have a trait that you can often implement automatically, or more complex code generation, as happens when you have a pattern that is repeated in the codebase like in the case of code inferred from the PostgreSQL schema, for instance methods that generate queries for foreign keys of a table, or methods which use available table indices to execute search queries.

The second kind of code generation, both for Rust and SQL, is primarily handled in the [`webcodegen`](`web/web_common/webcodegen`) crate. Whenever you identify a pattern that is repeated in the codebase, please consider whether it can be automated. If you are unsure, please open an issue on the GitHub repository and we will discuss it together.

### Dependency centralization

We try to keep the dependencies centralized in the [`Cargo.toml`](Cargo.toml) file of the root crate. This is to ensure that we can easily update dependencies and keep them in sync. If you need to add a new dependency, you most often will want to add it to the root crate. If you are unsure, please open an issue on the GitHub repository and we will discuss it together. In sub-crates, you will specify the dependency as a feature of the root crate using:

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

## Making a Pull Request

We try to follow the [GitHub Flow](https://docs.github.com/en/get-started/quickstart/github-flow) for making pull requests. This means that you should create a new branch for your changes, and then create a pull request from that branch to the main branch. Here follows a simple illustration of the process:

