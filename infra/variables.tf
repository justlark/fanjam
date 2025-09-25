variable "cloudflare_api_token" {
  type        = string
  description = "Cloudflare API token"
  sensitive   = true
}

variable "cloudflare_account_id" {
  type        = string
  description = "Cloudflare account ID"
}

variable "cloudflare_r2_access_key_id" {
  type        = string
  description = "R2 access key ID"
}

variable "cloudflare_r2_secret_access_key" {
  type        = string
  description = "R2 secret access key"
  sensitive   = true
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

# variable "upstash_api_token" {
#   type        = string
#   description = "Upstash API token"
#   sensitive   = true
# }

# variable "upstash_account_email" {
#   type        = string
#   description = "Upstash account email"
# }

variable "smtp_host" {
  type        = string
  description = "SMTP host"
}

variable "smtp_port" {
  type        = string
  description = "SMTP port"
}

variable "smtp_username" {
  type        = string
  description = "SMTP username"
  sensitive   = true
}

variable "smtp_password" {
  type        = string
  description = "SMTP password"
  sensitive   = true
}
