trunk serve\
	--port $TRUNK_PORT\
	--tls-key-path /home/appuser/app/web/nginx/${DOMAIN}-key.pem\
	--tls-cert-path /home/appuser/app/web/nginx/${DOMAIN}.pem\
	--address 0.0.0.0\
	--proxy-backend https://actix-server:${ACTIX_PORT}/api
