# FanJam

FanJam is an app that makes organizing conventions easier. Con organizers have
a shared dashboard where they can schedule panels, assign rooms, and make
announcements, and that information is shared with attendees in real time via a
web app.

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

You can use `just` to build and deploy the app. Run `just --list` to see a list
of recipes.

`just` commands that accept a `stage` accept either `prod` or `test`. This is
for infrastructure that is shared between tenant environments. `just` commands
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
just configure-secrets foo
```

Deploy NocoDB.

```
just deploy-env foo
```

Configure TLS certificates for NocoDB.

```
just configure-certs foo
```

View the NocoDB system user login credentials for the new environment:

```
just get-creds foo
```
