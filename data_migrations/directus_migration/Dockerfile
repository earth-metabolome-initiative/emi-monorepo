# Use the official PostgreSQL image
FROM postgres:17

# Set the environment variables for the default database and user
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword
ENV POSTGRES_DB=directus

# Copy the dump file to the container
COPY ./dump.sql /docker-entrypoint-initdb.d/dump.sql