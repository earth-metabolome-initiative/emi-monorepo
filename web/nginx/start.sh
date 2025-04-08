#!/bin/bash
echo "Starting Nginx with $1 configuration"
envsubst < /app/nginx.$1.conf.template | sed -e 's/§/$/g' > /etc/nginx/nginx.conf
cat /etc/nginx/nginx.conf > /app/nginx.$1.conf
nginx -g 'daemon off;'