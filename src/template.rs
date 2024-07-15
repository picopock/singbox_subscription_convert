pub const TEMPLATE: &str = r#"
{
  "log": {
    "level": "info",
    "timestamp": true
  },
  "dns": {
    "servers": [
      {
        "tag": "ali",
        "address": "https://223.5.5.5/dns-query",
        "address_strategy": "ipv4_only",
        "strategy": "ipv4_only",
        "detour": "DIRECT"
      },
      {
        "tag": "DNSPod",
        "address": "https://120.53.53.53/dns-query",
        "address_strategy": "ipv4_only",
        "strategy": "ipv4_only",
        "detour": "DIRECT"
      },
      {
        "tag": "google",
        "address": "https://8.8.8.8/dns-query",
        "address_strategy": "ipv4_only",
        "strategy": "ipv4_only",
        "detour": "🚀 节点选择"
      },
      {
        "tag": "cloudflare",
        "address": "https://1.1.1.1/dns-query",
        "address_strategy": "ipv4_only",
        "strategy": "ipv4_only",
        "detour": "🚀 节点选择"
      },
      {
        "tag": "block",
        "address": "rcode://success"
      }
    ],
    "rules": [
      {
        "outbound": [
          "any"
        ],
        "server": "ali"
      },
      {
        "clash_mode": "direct",
        "server": "ali"
      },
      {
        "clash_mode": "global",
        "server": "google"
      },
      {
        "rule_set": [
          "ads"
        ],
        "server": "block"
      },
      {
        "rule_set": [
          "microsoft-cn",
          "games-cn",
          "network-test",
          "applications",
          "cn",
          "private"
        ],
        "server": "ali"
      },
      {
        "rule_set": [
          "google-cn",
          "apple-cn",
          "proxy"
        ],
        "server": "google"
      }
    ],
    "final": "google",
    "strategy": "ipv4_only",
    "disable_cache": false,
    "disable_expire": false,
    "independent_cache": false,
    "reverse_mapping": false
  },
  "inbounds": [
    {
      "tag": "DNS-IN",
      "type": "direct",
      "listen": "0.0.0.0",
      "listen_port": 53
    },
    {
      "type": "tun",
      "tag": "TUN-IN",
      "inet4_address": [
        "198.18.0.1/16"
      ],
      "auto_route": true,
      "stack": "mixed",
      "sniff": true
    }
  ],
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
  ],
  "route": {
    "rules": [
      {
        "inbound": [
          "DNS-IN"
        ],
        "outbound": "DNS-OUT"
      },
      {
        "protocol": [
          "dns"
        ],
        "outbound": "DNS-OUT"
      },
      {
        "clash_mode": "global",
        "outbound": "🚀 节点选择"
      },
      {
        "clash_mode": "direct",
        "outbound": "DIRECT"
      },
      {
        "rule_set": [
          "ads"
        ],
        "outbound": "REJECT"
      },
      {
        "rule_set": [
          "microsoft-cn",
          "games-cn",
          "network-test",
          "applications",
          "cn",
          "cn-ip",
          "private-ip",
          "private"
        ],
        "outbound": "DIRECT"
      },
      {
        "domain_suffix": [
          "docker.io"
        ],
        "outbound": "🚀 节点选择"
      },
      {
        "rule_set": [
          "google-cn",
          "apple-cn",
          "proxy",
          "telegram-ip"
        ],
        "outbound": "🚀 节点选择"
      }
    ],
    "rule_set": [
      {
        "tag": "ads",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/ads.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "private",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/private.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "microsoft-cn",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/microsoft-cn.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "apple-cn",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/apple-cn.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "google-cn",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/google-cn.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "games-cn",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/games-cn.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "network-test",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/networktest.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "applications",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/applications.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "proxy",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/proxy.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "cn",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/cn.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "telegram-ip",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/telegramip.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "private-ip",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/privateip.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      },
      {
        "tag": "cn-ip",
        "type": "remote",
        "format": "binary",
        "url": "https://cdn.jsdelivr.net/gh/DustinWin/ruleset_geodata@sing-box-ruleset/cnip.srs",
        "download_detour": "🚀 节点选择",
        "update_interval": "1d"
      }
    ],
    "final": "🚀 节点选择",
    "auto_detect_interface": true
  },
  "experimental": {
    "cache_file": {
      "enabled": true
    },
    "clash_api": {
      "external_controller": "0.0.0.0:9090",
      "external_ui": "ui",
      "external_ui_download_url": "https://github.com/MetaCubeX/Yacd-meta/archive/gh-pages.zip",
      "external_ui_download_detour": "🚀 节点选择",
      "default_mode": "rule"
    }
  }
}
"#;
