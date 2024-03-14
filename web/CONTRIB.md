# Setting up the development environment to run the platform

## Adding the test domain to the hosts file
Among the first things to do is to add the test domain to the hosts file.
This is done by adding the following line to the file `/etc/hosts`:

```bash
sudo sh -c 'echo "127.0.0.1\temi.local" >> /etc/hosts'
```

Afterwards, you may need to flush the DNS cache:

```bash
sudo dscacheutil -flushcache
```

## Creating the certificates for SSL

Note that at this time we are only aware of a procedure for MacOS.

To create the certificates for SSL, you can use the following command:

```bash
brew install mkcert
```

After that, we want to also add `nss` since we want to test stuff on Firefox:

```bash
brew install nss
```

Next up, we need to install the root certificate. **This is a one-time operation and will require sudo**:
Run this command under the `/web` directory.

```bash
mkcert -install
```

Now, we can create the certificates for the platform:

```bash
mkcert -cert-file nginx/emi.local.pem -key-file nginx/emi.local-key.pem emi.local
```

## Setting up the environment variables
You need to setup the environment variables `.env`, starting from the `default_docker_env` file.

If you are in doubt about the values, you can ask the team for the correct values.

## Starting docker
To start the platform, you need to have Docker & Docker-compose.

Different platforms have different ways of installing Docker, so we will not cover that here.

## Starting Docker compose
To start the docker compose, you need to run the following command:

```bash
docker-compose -f docker-compose.yml up
```

To start the deployment version of the platform, you need to run the following command:

```bash
docker-compose -f docker-compose-deploy.yml up
```

### Integrity errors
Sometimes Trunk may start causing integrity errors. For example you may find in the console errors such as:

```bash
Failed to find a valid digest in the 'integrity' attribute for resource 'https://emi.local/frontend.js' with computed SHA-384 integrity '328Yb/77DVCU/r2WVi7/JLFi2UQE0ZOtdwEOg0zorekdvvT5nQIbXMf1uFWoXC95'. The resource has been blocked.
```

In such cases, stop and restart the Yew/Trunk docker.

### Upon encountering very odd errors
Sometimes it may happen that the Dockers lead to extremely odd errors. In those cases, to exclude the dockers, prune all the containers, images, and volumes. **DO THIS ON SYSTEMS WHERE YOU DO NOT HAVE OTHER DOCKERS**.

```bash
sudo rm -fdr ./database_tmp # This is the database volume
sudo rm -fdr ./redis # This is the redis volume
sudo rm -fdr ./frontend/dist # This is the frontend volume
docker system prune --all
```