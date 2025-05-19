# Example extension

[![Build Example Extension](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-example-extension.yml/badge.svg)](https://github.com/earth-metabolome-initiative/emi-monorepo/actions/workflows/pgrx-example-extension.yml)

To build this example extension, just run the following docker build command:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
