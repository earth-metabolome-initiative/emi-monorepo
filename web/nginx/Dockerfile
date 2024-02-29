# Use an Nginx base image
FROM nginx:latest

WORKDIR /app

# Start Nginx
CMD envsubst < /app/nginx.conf.template | sed -e 's/§/$/g' > /etc/nginx/nginx.conf && \
    cat /etc/nginx/nginx.conf > /app/nginx.conf && \
    nginx -g 'daemon off;'