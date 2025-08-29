source ./config.nu

def get-api-base [stage_name: string] {
  let config = get-global-config
  let api_urls = $config.stages | where "name" == $stage_name | get "api_url"

  match ($api_urls | length) {
    0 => {
      print --stderr $"No such stage: ($stage_name)"
      exit 1
    }
    1 => {
      let api_url = $api_urls | first
      return $api_url
    }
    _ => {
      print --stderr $"Multiple stages match: ($stage_name)"
      exit 1
    }
  }
}

def get-api-headers [stage_name: string] {
  get-tofu-env | load-env

  let admin_api_tokens = tofu -chdir=./infra/ output -json worker_admin_api_tokens | from json
  let admin_api_token = $admin_api_tokens | get $stage_name

  ["Authorization", $"Bearer ($admin_api_token)"]
}

def "admin-api get" [stage_name: string, endpoint: string] {
  let api_endpoint = $"(get-api-base $stage_name)($endpoint)"
  let headers = get-api-headers $stage_name

  http get --headers $headers $api_endpoint
}

def "admin-api post" [stage_name: string, endpoint: string, body: any = ""] {
  let api_endpoint = $"(get-api-base $stage_name)($endpoint)"
  let headers = get-api-headers $stage_name

  http post --content-type "application/json" --headers $headers $api_endpoint $body
}

def "admin-api put" [stage_name: string, endpoint: string, body: any = ""] {
  let api_endpoint = $"(get-api-base $stage_name)($endpoint)"
  let headers = get-api-headers $stage_name

  http put --content-type "application/json" --headers $headers $api_endpoint $body
}

def "admin-api delete" [stage_name: string, endpoint: string] {
  let api_endpoint = $"(get-api-base $stage_name)($endpoint)"
  let headers = get-api-headers $stage_name

  http delete --headers $headers $api_endpoint
}
