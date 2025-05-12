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
  npx wrangler --env test dev

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

# generate the configuration for an environment
[group("manage infrastructure")]
configure-env env:
  ./tools/configure-env.nu {{ env }}

# create a new NocoDB instance
[group("manage infrastructure")]
create-env env:
  fly -c ./infra/environments/{{ env }}/fly.yaml launch --org sparklefish --copy-config --yaml --no-deploy --yes

# pass secrets to the NocoDB instance
[group("manage infrastructure")]
configure-secrets env:
  ./tools/configure-secrets.nu {{ env }}

# deploy an existing NocoDB instance
[group("manage infrastructure")]
deploy-env env:
  fly -c ./infra/environments/{{ env }}/fly.yaml deploy

# generate TLS certificates for an environment
[group("manage infrastructure")]
configure-certs env:
  fly -c ./infra/environments/{{ env }}/fly.yaml certs add {{ env }}.fanjam.live

# get the system user login credentials for an environment
[group("manage infrastructure")]
get-creds env:
  ./tools/get-creds.nu {{ env }}

# configure an environment with the NocoDB API token
[group("manage environments")]
set-noco-token stage env:
  ./tools/set-noco-token.nu {{ stage }} {{ env }}

# create a new empty NocoDB base in an environment
[group("manage environments")]
create-noco-base stage env:
  ./tools/create-noco-base.nu {{ stage }} {{ env }}

# generate a new app link for an environment
[group("manage environments")]
generate-app-link stage env:
  ./tools/generate-app-link.nu {{ stage }} {{ env }}

# initialize a new environment
[group("manage environments")]
init-env stage env: (set-noco-token stage env) (create-noco-base stage env) (generate-app-link stage env)
