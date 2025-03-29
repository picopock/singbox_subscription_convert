[中文指南](https://blog.picopock.com/2023/02/12/debian/debian/#%E9%83%A8%E7%BD%B2-singBox-%E5%AE%B9%E5%99%A8)

# singbox_subscription_convert

A tools for converting clash subscription to singbox config.

> Note: Only test vmess and ss protocol.

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

Currently, the tool only modifies the items of type `urltest` and `selector` in the `outbounds` configuration of the template, and appends the nodes of all subscriptions to the end of `outbounds`.

For the `urltest` type, modify the `outbounds` field according to the interpolation rules.
For the `selector` type, modify the `outbounds` and `default` fields according to the interpolation rules.

Traverse the `outbounds` fields of the corresponding types:
  If there is a `{{ALL-TAG}} `field, fill in all the node names of the obtained subscription at the current position, replacing the placeholder rule field.
  If there are interpolation rules such as `{{EXCLUDE-TAG:xxx,yyy,zzz}}`, `{{INCLUDE-TAG:xxx,yyy,zzz}}`, or `{{EXCLUDE-TAG:xxx,yyy,zzz;INCLUDE-TAG:xxx,yyy,zzz}}`, perform fuzzy matching:
    `EXCLUDE-TAG`: Represents an exclusion rule, followed by fuzzy matching rules separated by `,`. The fuzzy matching rules are in an OR relationship.
    `INCLUDE-TAG`: Represents an inclusion rule, followed by fuzzy matching rules separated by `,`. The fuzzy matching rules are in an OR relationship.
    If `EXCLUDE-TAG` and `INCLUDE-TAG` exist simultaneously, they need to be separated by `;` without regard to order. The two rules are in an AND relationship and need to be satisfied simultaneously.

An example configuration is as follows:

```json
{
  "outbounds": [
    {
      "tag": "DIRECT",
      "type": "direct"
    },
    {
      "tag": "🚀 Select",
      "type": "selector",
      "interrupt_exist_connections": true,
      "default": "♻️ AutoSelect",
      "outbounds": [
        "♻️ AutoSelect",
        "🚀 ManualSelect"
      ]
    },
    {
      "tag": "♻️ AutoSelect",
      "type": "urltest",
      "url": "https://www.gstatic.com/generate_204",
      "interval": "180s",
      "tolerance": 50,
      "idle_timeout": "5m",
      "interrupt_exist_connections": true,
      "outbounds": ["{{ALL-TAG}}"]
    },
    {
      "tag": "🚀 ManualSelect",
      "type": "selector",
      "interrupt_exist_connections": true,
      "default": "{{INCLUDE-TAG:xxx,yyy,zzz}}",
      "outbounds": ["{{ALL-TAG}}"]
    },
    {
      "type": "urltest",
      "tag": "🎵 TikTok",
      "url": "https://www.gstatic.com/generate_204",
      "interval": "30s",
      "tolerance": 50,
      "idle_timeout": "5m",
      "interrupt_exist_connections": true,
      "outbounds": ["{{EXCLUDE-TAG:aaa,bbb,ccc;INCLUDE-TAG:xxx,yyy,zzz}}"]
    }
  ]
}
```