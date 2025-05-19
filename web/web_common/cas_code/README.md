# Chemical Abstracts Service (CAS) code

Rust crate providing a structured representation of [CAS Registry Numbers (CAS RN)](https://en.wikipedia.org/wiki/CAS_Registry_Number) and related functionality.

To build this extension, just run the following docker build command:

```bash
USER_ID=$(id -u) GROUP_ID=$(id -g) docker compose up
```

Note that the `USER_ID` and `GROUP_ID` environment variables are used to set the user and group IDs inside the Docker container to match those of the host system. This is important for file permissions when mounting volumes and avoid writing out files with root permissions.
