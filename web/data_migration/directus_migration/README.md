
docker compose up

pg_restore -U myuser -d mydatabase /docker-entrypoint-initdb.d/dump.sql

docker ps
docker attach 540af6df5cc3