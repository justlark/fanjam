#!/usr/bin/env nu

def main [host: string] {
  psql --dbname noco --host $host --port 5432 --username sparklefish
}
