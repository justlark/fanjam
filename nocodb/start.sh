#!/bin/sh

set -e

env PORT=8081 /usr/src/appEntry/start.sh &

exec caddy run --config /etc/caddy/Caddyfile --adapter caddyfile
