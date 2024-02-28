# Use an Nginx base image
FROM nginx:latest

WORKDIR /app

# Copy the Nginx configuration file
COPY nginx.conf.template /app/nginx.conf.template

RUN envsubst < /app/nginx.conf.template > /etc/nginx/nginx.conf

# Start Nginx
CMD envsubst < /app/nginx.conf.template | sed -e 's/§/$/g' > /etc/nginx/nginx.conf && \
    cat /etc/nginx/nginx.conf > /app/nginx.conf && \
    nginx -g 'daemon off;'