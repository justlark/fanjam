terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.52"
    }

    neon = {
      source  = "kislerdm/neon"
      version = "~> 0.9"
    }

    upstash = {
      source  = "upstash/upstash"
      version = "~> 1.5"
    }
  }

  cloud {
    organization = "sparklefish"
    hostname     = "app.terraform.io"

    workspaces {
      name = "sparklefish"
    }
  }
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

provider "neon" {
  api_key = var.neon_api_token
}

provider "upstash" {
  email   = var.upstash_account_email
  api_key = var.upstash_api_token
}
