# Font Awesome Icons

A crate providing a rust enumeration with all Font Awesome icons.

This crate additional provides bindings for `diesel` which allow you to use the icons as an enum in PostgreSQL, and in SQLite as a TEXT column.

Furthermore, the crate also provides `pgrx`-based bindings that allow you to use these icons as a PostgreSQL enum type in your database, which you can install as an extension.

*This library is in no way endorsed or supported by FontAwesome. We just needed a precise way to use FontAwesome icons in our Rust code, and this is the result.*

## Compiling the PGRX extension

After having cloned the repository, you can compile the PGRX extension in the `./extension` directory by running in this directory:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
