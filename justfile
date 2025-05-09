# install npm dependencies for the client
[working-directory: "./client/"]
_install-client:
  npm install

# run the client locally
[working-directory: "./client/"]
[group("test locally")]
run-client: _install-client
  npm run dev

# run the server locally
[group("test locally")]
[working-directory: "./server/"]
run-server:
  npx wrangler --env test dev --remote

# deploy the client
[working-directory: "./client/"]
[group("deploy changes")]
[confirm]
deploy-client stage: _install-client
  npm run deploy:{{ stage }}

# deploy the server
[working-directory: "./server/"]
[group("deploy changes")]
[confirm]
deploy-server stage:
  npx wrangler deploy --env {{ stage }}

# redeploy an existing NocoDB instance
[group("manage infrastructure")]
deploy-nocodb env:
  fly -c ./infra/environments/{{ env }}/fly.toml deploy

# create a new NocoDB instance
[group("manage infrastructure")]
create-nocodb env:
  fly -c ./infra/environments/{{ env }}/fly.toml launch --org sparklefish --copy-config --no-deploy --yes

# generate the configuration for a new environment
[group("manage infrastructure")]
generate-env-config env:
  ./tools/create-env.nu {{ env }}
