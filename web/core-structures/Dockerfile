# Use the official PostgreSQL image from the Docker Hub
FROM postgres:17

# Install PostGIS
RUN apt-get update && \
    apt-get install -y postgis postgresql-17-postgis-3 && \
    apt-get install -y postgresql-17-cron

COPY ./extension/usr/lib/postgresql/17/lib/pgrx_validation.so /usr/lib/postgresql/17/lib/pgrx_validation.so
COPY ./extension/usr/share/postgresql/17/extension/pgrx_validation.control /usr/share/postgresql/17/extension/pgrx_validation.control
COPY ./extension/usr/share/postgresql/17/extension/pgrx_validation--0.0.0.sql usr/share/postgresql/17/extension/pgrx_validation--0.0.0.sql
