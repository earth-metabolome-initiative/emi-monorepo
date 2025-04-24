trunk serve\
	--port $TRUNK_PORT\
	--tls-key-path ../nginx/${DOMAIN}-key.pem\
	--tls-cert-path ../nginx/${DOMAIN}.pem\
	--address 0.0.0.0\
	--proxy-backend https://actix-server:${ACTIX_PORT}/api
