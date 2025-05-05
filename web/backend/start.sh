# We check whether the `/home/appuser/app/web/nginx/rootCA.pem` and the `/home/appuser/app/web/nginx/rootCA-key.pem` files exist
if [ ! -f /home/appuser/app/web/nginx/rootCA.pem ] || [ ! -f /home/appuser/app/web/nginx/rootCA-key.pem ]; then
	# If they don't exist, we create them
	cp -r ${mkcert -CAROOT} /home/appuser/app/web/nginx
else
	# If they exist, we copy them to the `${mkcert -CAROOT}` directory
	# so to make sure that the same rootCA is used
	cp /home/appuser/app/web/nginx/rootCA.pem ${mkcert -CAROOT}
	cp /home/appuser/app/web/nginx/rootCA-key.pem ${mkcert -CAROOT}
fi

# We copy the CAROOT certificate outside the container to use it
CAROOT=/home/appuser/app/web/nginx mkcert -install
CAROOT=/home/appuser/app/web/nginx mkcert -cert-file /home/appuser/app/web/nginx/${DOMAIN}.pem -key-file /home/appuser/app/web/nginx/${DOMAIN}-key.pem ${DOMAIN}
RUST_LOG=debug cargo watch -q -c -w src/ -x run
