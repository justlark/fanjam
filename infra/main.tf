terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 5.4"
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
