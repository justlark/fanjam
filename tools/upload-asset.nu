#!/usr/bin/env nu

source ./config.nu

def main [env_name: string, asset_name: string] {
  # Make sure an environment with this name exists.
  get-env-config $env_name

  npx wrangler@latest --config "./client/wrangler.toml" r2 object put --remote --pipe $"sparklefish-assets/env/($env_name)/($asset_name)"
}
