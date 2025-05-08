# Install npm dependencies for the client
[working-directory: "./client/"]
_install-client:
  npm install

# Run the client locally
[working-directory: "./client/"]
run-client: _install-client
  npm run dev

# Run the server locally
[working-directory: "./server/"]
run-server:
  npx wrangler --env test dev --remote

# Deploy the client
[working-directory: "./client/"]
deploy-client env: _install-client
  npm run deploy:{{ env }}

# Deploy the server
[working-directory: "./server/"]
deploy-server env:
  npx wrangler deploy --env {{ env }}

# Deploy NocoDB
[working-directory: "./fly/"]
deploy-nocodb env:
  fly -c ./noco-{{ env }}.toml deploy
