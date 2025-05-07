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

Run the client locally:

```
cd ./client/
npm install
npm run dev
```

Run the server locally:

```
cd ./server/
npx wrangler --env test dev --remote
```

## Deployment

Deploy the server to the test environment:

```shell
cd ./server/
npx wrangler deploy --env test
```

Deploy the server to the prod environment:

```shell
cd ./server/
npx wrangler deploy --env prod
```

Deploy the client to the test environment:

```
cd ./client/
npm install
npm run deploy:test
```

Deploy the client to the prod environment:

```
cd ./client/
npm install
npm run deploy:prod
```

## Setup

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
