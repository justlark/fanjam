#!/usr/bin/env nu

source ./config.nu

def main [env_name: string, asset_name: string] {
  let env_config = get-env-config $env_name
  npx wrangler@latest --config "./server/wrangler.toml" r2 object put --remote --pipe $"sparklefish-assets-($env_config.stage)/env/($env_name)/($asset_name)"
}
