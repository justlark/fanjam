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

# initialize a NocoDB instance with a new base
[group("manage infrastructure")]
setup-env stage env:
  ./tools/setup-env.nu {{ stage }} {{ env }}
