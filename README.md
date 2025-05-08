# sparklefish

Sparklefish is the codename for a yet-to-be-named app designed for con
organizers and attendees to make organizing and navigating cons easier.

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

To work in this repo, you'll need to install:

- [just](https://github.com/casey/just?tab=readme-ov-file#installation)
- [Rust](https://www.rust-lang.org/tools/install)
- [npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [flyctl](https://fly.io/docs/flyctl/install/)

You can use `just` to build and deploy the app. Run `just --list` to see a list
of recipes.

There are two environments this app can be deployed in: `test` and `prod`.
You'll need to pass one of these to `just` commands that accept an `env`
parameter.

## Quirks

When setting up NocoDB, there are some quirks with the Redis and Postgres
connection strings that are not addressed in the NocoDB documentation.

The Postgres connection string should look like this. The `ssl=true` is
necessary for NocoDB to talk to Neon.

```
pg://DOMAIN:PORT?u=USER&p=PASSWORD&d=DATABASE&ssl=true
```

The Redis connection string should look like this. The `family=6` is necessary
for NocoDB to talk to Upstash.

```
redis://USER:PASSWORD@DOMAIN:PORT/?family=6
```
