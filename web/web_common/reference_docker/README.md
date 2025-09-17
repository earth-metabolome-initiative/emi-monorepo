# Reference docker

Crate providing a simple `testcontainer`-based docker image.
In our case, the postgres image is currently stored within the `web` folder.

```bash
docker build --tag mycustom/postgres-postgis:17.4 --file ./PostgresDockerfile .
```
