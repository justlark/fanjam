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
  }

  backend "pg" {}
}

provider "cloudflare" {
  api_token = var.cloudflare_api_token
}

provider "neon" {
  api_key = var.neon_api_token
}
