variable "cloudflare_api_token" {
  type        = string
  description = "Cloudflare API token"
  sensitive   = true
}

variable "cloudflare_account_id" {
  type        = string
  description = "Cloudflare account ID"
}

variable "neon_api_token" {
  type        = string
  description = "Neon API token"
  sensitive   = true
}

variable "neon_org_id" {
  type        = string
  description = "Neon organization ID"
}
