# Use an Nginx base image
FROM nginx:latest

RUN apt-get update && apt-get install -y tmux

WORKDIR /app

# Start Nginx
CMD tmux new-session -s nginx 'bash /app/start.sh'

