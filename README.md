[中文指南](https://blog.picopock.com/2023/02/12/debian/debian/#%E9%83%A8%E7%BD%B2-singBox-%E5%AE%B9%E5%99%A8)

# singbox_subscription_convert

A tools for partially converting clash subscription to singbox config.

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
The first six configurations are fixed. After the tool obtains the clash subscription, it modifies the fifth and sixth configurations.  
It fills the outbounds field for the urltest type configuration and fills the outbounds and default fields for the selector type configuration. The default field takes the first subscription tag by default.

The first six configurations of outbounds are as follows：

```json
"outbounds": [
  {
    "tag": "DNS-OUT",
    "type": "dns"
  },
  {
    "tag": "DIRECT",
    "type": "direct"
  },
  {
    "tag": "REJECT",
    "type": "block"
  },
  {
    "tag": "🚀 节点选择",
    "type": "selector",
    "interrupt_exist_connections": true,
    "default": "♻️ 自动选择",
    "outbounds": [
      "♻️ 自动选择",
      "🚀 手动切换"
    ]
  },
  {
    "tag": "♻️ 自动选择",
    "type": "urltest",
    "url": "https://www.gstatic.com/generate_204",
    "interval": "3m",
    "tolerance": 50,
    "idle_timeout": "50m",
    "interrupt_exist_connections": true,
    "outbounds": []
  },
  {
    "tag": "🚀 手动切换",
    "type": "selector",
    "interrupt_exist_connections": true,
    "default": "",
    "outbounds": []
  }
]
```