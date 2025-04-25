# Use an Nginx base image
FROM nginx:latest

RUN apt update && apt install -y tmux

WORKDIR /app

# Start Nginx
CMD tmux new-session -s nginx 'bash /app/start.sh develop'
