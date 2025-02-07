[中文指南](https://blog.picopock.com/2023/02/12/debian/debian/#%E9%83%A8%E7%BD%B2-singBox-%E5%AE%B9%E5%99%A8)

# singbox_subscription_convert

A tools for partially converting clash subscription to singbox config.

> Note: Only support vmess protocol convert at now.

## Usage

```sh
subscription_convert <subscribe address> [-t <template path>] [-o <output path>]
```

- `subscribe address`

  The tool will automatically append the `flag=clash` parameter to the subscription address to get the YAML format subscription.

- `-t <template path>`

  Specify a JSON template for generating sing-box. If not specified, the tool's built-in template will be used.

- `-o <output path>`

  Specify the output directory or file path for the generated sing-box configuration.

## Subscription Update Rules

The tool only modifies the outbounds configuration.  
It fills the outbounds field for the urltest type configuration and fills the outbounds and default fields for the selector type configuration. The default field takes the first subscription tag by default.