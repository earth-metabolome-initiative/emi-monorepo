# We copy the CAROOT certificate outside the container to use it
cp -r ${mkcert -CAROOT} /home/appuser/app/web/nginx
CAROOT=/home/appuser/app/web/nginx mkcert -install
CAROOT=/home/appuser/app/web/nginx mkcert -cert-file /home/appuser/app/web/nginx/${DOMAIN}.pem -key-file /home/appuser/app/web/nginx/${DOMAIN}-key.pem ${DOMAIN}
RUST_LOG=debug cargo watch -q -c -w src/ -x run
