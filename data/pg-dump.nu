#!/usr/bin/env nu

def main [host: string, schema: string] {
  pg_dump --data-only --dbname noco --host $host --port 5432 --username sparklefish --table $"($schema).*"
}
