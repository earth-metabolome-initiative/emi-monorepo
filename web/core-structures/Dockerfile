# Use the official PostgreSQL image from the Docker Hub
FROM postgres:17

# Install PostGIS
RUN apt-get update && \
    apt-get install -y postgis postgresql-17-postgis-3 && \
    apt-get install -y postgresql-17-cron
