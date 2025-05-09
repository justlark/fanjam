# FanJam

FanJam is an app that makes organizing conventions easier. Con organizers have
a shared dashboard where they can schedule panels, assign rooms, and make
announcements, and that information is shared with attendees in real time via a
web app.

## Stack

- The client is written in Vue and TypeScript and hosted on [Cloudflare
  Workers](https://developers.cloudflare.com/workers/).
- The backend is written in Rust and hosted on Cloudflare Workers.
- The database and interface for con organizers is provided by a
  [NocoDB](https://nocodb.com/) instance hosted on [Fly.io](https://fly.io/).
- The Postgres provider for NocoDB is [Neon](https://neon.tech).
- The Redis provider for NocoDB is [Upstash](https://upstash.com/).
- The object storage provider for NocoDB is [Cloudflare
  R2](https://developers.cloudflare.com/r2/).
- The SMTP provider for NocoDB is [MailerSend](https://www.mailersend.com/).

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

This project is referred to by the codename "sparklefish" throughout the
codebase and infrastructure.

## Deployment

These are the instructions for deploying a new instance of FanJam at
`https://foo.fanjam.live`.

Start by generating the necessary configuration for the environment.

```
just generate-env-config foo
```

This will generate some files. Check them into the repo.

Create a new app in Fly.io for the NocoDB instance.

```
just create-nocodb foo
```

Deploy the supporting infrastructure using Terraform.

```
cd ./infra/
terraform apply
```

Dump the environment secrets from Terraform.

```
cd ./infra/
terraform output secrets
```

Pass secrets for the environment into NocoDB.

```
fly secrets set --app sparklefish-noco-foo NAME=VALUE
```

Deploy NocoDB.

```
just deploy-nocodb foo
```

Configure TLS certificates for NocoDB.

```
fly certs add --app sparklefish-noco-foo foo.fanjam.live
```
