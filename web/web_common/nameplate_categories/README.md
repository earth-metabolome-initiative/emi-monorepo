# Nameplate Categories

Crate providing a nameplate categories enumeration, with integration to [`diesel`](https://github.com/diesel-rs/diesel) with both `SQLite` and `PostgreSQL` backends, and additionally support for [`pgrx`](https://github.com/pgcentralfoundation/pgrx).

## Compiling the PGRX extension

After having cloned the repository, you can compile the PGRX extension in the `./extension` directory by running in this directory:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
