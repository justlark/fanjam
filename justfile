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
deploy-client stage: _install-client
  npm run deploy:{{ stage }}

# Deploy the server
[working-directory: "./server/"]
deploy-server stage:
  npx wrangler deploy --env {{ stage }}

# Redeploy an existing NocoDB instance
deploy-nocodb env:
  fly -c ./infra/environments/{{ env }}/fly.toml deploy

# Create a new NocoDB instance
create-nocodb env:
  fly -c ./infra/environments/{{ env }}/fly.toml launch --org sparklefish --copy-config --no-deploy --yes
