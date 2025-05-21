# FanJam

FanJam is a free event planning app for conventions. Con organizers have a
dashboard where they can schedule panels, assign rooms, and make announcements,
and that information is shared with attendees in real time via a web app.

## Architecture

- The client is written in Vue and TypeScript and hosted on [Cloudflare
  Workers](https://developers.cloudflare.com/workers/).
- The backend is a serverless function written in Rust and hosted on Cloudflare
  Workers.
- The storage provider for the backend is [Cloudflare Workers
  KV](https://developers.cloudflare.com/kv/).
- The database and interface for con organizers is provided by a
  [NocoDB](https://nocodb.com/) instance hosted on [Fly.io](https://fly.io/).
- The Postgres provider for NocoDB is [Neon](https://neon.tech).
- The Redis provider for NocoDB is [Upstash](https://upstash.com/).
- The object storage provider for NocoDB is [Cloudflare
  R2](https://developers.cloudflare.com/r2/).
- The SMTP provider for NocoDB is [MailerSend](https://www.mailersend.com/).

FanJam is single-tenant, meaning we have a separate NocoDB instance, Postgres
cluster, Redis database, and R2 bucket per tenant environment.

We have a single deployment of the frontend and a single deployment of the
backend which are shared across tenant environments. We have separate `prod`
and `test` deployments of each.

## Development

To build and run the app, you'll need to install:

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)

To deploy the app, you'll additionally need to install:

- [Nu](https://www.nushell.sh/book/installation.html)
- [flyctl](https://fly.io/docs/flyctl/install/)
- [Terraform](https://developer.hashicorp.com/terraform/install)

You can use `just` to build and deploy the app. Run `just` to see a list of
recipes.

`just` recipes that accept a `stage` accept either `prod` or `test`. This is
for infrastructure that is shared between tenant environments. `just` recipes
that accept an `env` accept the name of a tenant environment.

This project is referred to by the codename "sparklefish" throughout the
codebase and infrastructure.

## Deployment

These are the instructions for deploying a new instance of FanJam at
`https://foo.fanjam.live`.

Start by generating the necessary configuration for the environment.

```
just configure-env foo
```

This will generate some files, which can be edited as necessary. Check them
into the repo.

Create a new app in Fly.io for the NocoDB instance.

```
just create-env foo
```

Deploy the supporting infrastructure using Terraform.

```
cd ./infra/
terraform plan
terraform apply
```

Pass secrets for the environment into NocoDB.

```
just deploy-secrets foo
```

Deploy NocoDB.

```
just deploy-env foo
```

Configure TLS certificates for NocoDB.

```
just deploy-certs foo
```

View the NocoDB system user login credentials for the new environment.

```
just get-creds foo
```

At this point, you'll need to log into the NocoDB instance manually to generate
an API token.

Finally, initialize the NocoDB instance with a new base. You'll need to specify
which deployment of the backend you want to use. It will prompt you for the API
token interactively. Once you've done this, you can lose the token.

```
just init-env foo
```

## NocoDB

We maintain a fork of NocoDB for FanJam at
[justlark/nocodb](https://github.com/justlark/nocodb).

We'll want to periodically pull in changes from upstream. Once you've cloned
the repo, rebased `master` onto the latest upstream release, and resolved any
merge conflicts, you'll need to deploy the images. These are the instructions
for doing that.

First, build the image.

```
./build-local-docker-image.sh
```

This may generate some files, which should be committed to the repo. This will
generate a local image named `nocodb-local`.

Tag the image with both `latest` _and_ the version number of the upstream
release. Test environments (like [playground](https://playground.fanjam.live))
should be tagged with `latest`. Production environments should be pinned to a
release.

```
docker tag nocodb-local ghcr.io/justlark/nocodb:latest

# Use the actual version number!
docker tag nocodb-local ghcr.io/justlark/nocodb:v0.263.4
```

Then push to the container registry.

```
docker push ghcr.io/justlark/nocodb:latest

# Use the actual version number!
docker push ghcr.io/justlark/nocodb:v0.263.4
```

Redeploy a test environment (like `playground`) for testing using `just
deploy-env playground`. Once you're confident in the release, update the
`fly.yaml` of all the environments you want to upgrade, then redeploy them.

Finally, update [config.yaml](./config.yaml) with the latest version tag
(**not** `latest`).
