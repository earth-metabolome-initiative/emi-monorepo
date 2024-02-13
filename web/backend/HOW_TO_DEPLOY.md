# How to deploy

## Preliminary steps
In order to deploy the application, you need to have the following installed on your machine:

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)

### Setup for macOS
If you are running on a macOS machine, after having installed [`brew`](https://brew.sh/), you can run the following commands to install the required software:

```bash
brew install docker
brew install docker-compose
brew install git
```

### Setup for Ubuntu
If you are running on an Ubuntu machine, you can run the following commands to install the required software:

```bash
sudo apt-get update
sudo apt-get install docker
sudo apt-get install docker-compose
sudo apt-get install git
```

## Cloning the repository
Once you have installed the required software, you can clone the repository by running the following command:

```bash
git clone git@github.com:emikg/emikg.git
```

## Customizing the environemnt file
In order to run the docker compose, you need to create a `.env` file in the root directory of the project. You can use the [`default_env`](https://github.com/emikg/emikg/blob/main/default_env) file as a template.

Once you have fully customized the `.env` file, you can run the docker compose command to start the application.

### Setting up the remote backup server
Within the `.env` file, you can also set up the remote backup server. In order to do so, you need to set the following variables:

- `BACKUP_SERVER_USERNAME`: the username of the user on the remote server
- `BACKUP_SERVER_NAME`: the name of the remote server
- `BACKUP_SERVER_PORT`: the port of the remote server
- `BACKUP_SERVER_PRIVATE_KEY_PATH`: the path to the private key for the remote server
- `BACKUP_REMOTE_PATH`: the path to the remote backup directory

The backup server needs to be a Linux server with `rsync` installed. The backup server also needs to have a directory where the backups will be stored.

## Starting the docker services

### Starting the docker service on macOS
If you are running on a macOS machine, you need to start the docker service by running the following command:

```bash
open --background -a Docker
```

### Starting the docker service on Ubuntu
If you are running on an Ubuntu machine, you need to start the docker service by running the following command:

```bash
sudo systemctl start docker
```

## Running the docker compose
To run the docker compose, you need to run the following command in the root directory of the project:

```bash
docker-compose up -d --build -V
```

Note that, in some systems, the `docker-compose` command might not be available and it may be integrated into the `docker` command. In this case, you need to run the following command:

```bash
docker compose up -d --build -V
```

Additionally, if you want to run the version with also the backup service enabled, you can run the following command:

```bash
docker-compose --profile backup up -d --build -V
```

### Common errors encountered when running the docker compose
In the following sections, we will describe some of the common errors that you might encounter when running the docker compose.

#### MacOS error caused by not having started the docker desktop application
If you are running on a macOS machine, you might encounter the following error:

```text
Traceback (most recent call last):
  File "urllib3/connectionpool.py", line 670, in urlopen
  File "urllib3/connectionpool.py", line 392, in _make_request
  File "http/client.py", line 1255, in request
  File "http/client.py", line 1301, in _send_request
  File "http/client.py", line 1250, in endheaders
  File "http/client.py", line 1010, in _send_output
  File "http/client.py", line 950, in send
  File "docker/transport/unixconn.py", line 43, in connect
FileNotFoundError: [Errno 2] No such file or directory
```

This error is caused by not having started the docker desktop application. Do start your docker desktop application and retry to run the docker compose after that.

#### MacOS error caused by not having waited enough after having started the docker service
If you are running on a macOS machine, you might encounter the following error:

```text
Error response from daemon: dial unix docker.raw.sock: connect: no such file or directory
```

This error is caused by not having started the docker desktop application. Note that it can take a minute, so even if you have just started the docker desktop application, you might still encounter this error. Do wait a minute and retry to run the docker compose after that.
