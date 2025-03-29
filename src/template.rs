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
        "type": "https",
        "server": "223.5.5.5",
        "detour": "DIRECT"
      },
      {
        "tag": "DNSPod",
        "type": "https",
        "server": "120.53.53.53",
        "detour": "DIRECT"
      },
      {
        "tag": "google",
        "type": "https",
        "server": "8.8.8.8",
        "detour": "🚀 节点选择"
      },
      {
        "tag": "cloudflare",
        "type": "https",
        "server": "1.1.1.1",
        "detour": "🚀 节点选择"
      },
      {
        "tag": "block",
        "type": "predefined",
        "responses": [
          {
            "rcode": "NOERROR"
          }
        ]
      }
    ],
    "rules": [
      {
        "domain": [
          "sh.picopock.com",
          "gs.picopock.com",
          "appstorrent.ru",
          "kp.m-team.cc"
        ],
        "domain_keyword": [
          "m-team"
        ],
        "strategy": "ipv4_only",
        "action": "route",
        "server": "ali"
      },
      {
        "domain_suffix": [
          "creaders.net"
        ],
        "domain_keyword": [
          "mikrotik"
        ],
        "strategy": "ipv4_only",
        "action": "route",
        "server": "google"
      },
      {
        "clash_mode": "direct",
        "strategy": "ipv4_only",
        "action": "route",
        "server": "ali"
      },
      {
        "clash_mode": "global",
        "strategy": "ipv4_only",
        "action": "route",
        "server": "google"
      },
      {
        "rule_set": "ads",
        "action": "reject",
        "method": "default",
        "no_drop": false
      },
      {
        "domain": [
          "jd.com",
          "taobao.com",
          "appstorrent.ru"
        ],
        "domain_suffix": [
          "cdn.jiashule.com",
          "cdn.jsdelivr.net",
          "jsdelivr.map.fastly.net",
          "adobe.com",
          "jd.com",
          "taobao.com"
        ],
        "domain_keyword": [
          "adobe"
        ],
        "strategy": "ipv4_only",
        "action": "route",
        "server": "ali"
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
        "strategy": "ipv4_only",
        "action": "route",
        "server": "ali"
      },
      {
        "rule_set": [
          "google-cn",
          "apple-cn",
          "proxy"
        ],
        "strategy": "ipv4_only",
        "action": "route",
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
      "type": "direct",
      "tag": "DNS-IN",
      "listen": "0.0.0.0",
      "listen_port": 53
    },
    {
      "type": "tun",
      "tag": "TUN-IN",
      "address": [
        "198.18.0.1/16"
      ],
      "auto_route": true,
      "stack": "mixed",
      "sniff": true
    }
  ],
  "outbounds": [
    {
      "tag": "DIRECT",
      "type": "direct"
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
      "outbounds": ["{{ALL-TAG}}"]
    },
    {
      "tag": "🚀 手动切换",
      "type": "selector",
      "interrupt_exist_connections": true,
      "default": "",
      "outbounds": ["{{ALL-TAG}}"]
    }
  ],
  "route": {
    "default_domain_resolver": {
      "server": "ali"
    },
    "rules": [
      {
        "action": "sniff"
      },
      {
        "action": "hijack-dns",
        "protocol": "dns"
      },
      {
        "domain": [
          "sh.picopock.com",
          "gs.picopock.com",
          "appstorrent.ru",
          "kp.m-team.cc"
        ],
        "domain_suffix": [
          "cdn.jiashule.com",
          "xiaoliyu.cyou",
          "xiaoliyu.xyz",
          "jd.com",
          "taobao.com",
          "argotunnel.com"
        ],
        "domain_keyword": [
          "m-team"
        ],
        "action": "route",
        "outbound": "DIRECT"
      },
      {
        "domain_suffix": [
          "creaders.net"
        ],
        "domain_keyword": [
          "mikrotik"
        ],
        "action": "route",
        "outbound": "🚀 节点选择"
      },
      {
        "clash_mode": "global",
        "action": "route",
        "outbound": "🚀 节点选择"
      },
      {
        "clash_mode": "direct",
        "action": "route",
        "outbound": "DIRECT"
      },
      {
        "rule_set": "ads",
        "action": "reject",
        "method": "default",
        "no_drop": false
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
        "action": "route",
        "outbound": "DIRECT"
      },
      {
        "domain_suffix": [
          "v2ex.com",
          "docker.io"
        ],
        "action": "route",
        "outbound": "🚀 节点选择"
      },
      {
        "rule_set": [
          "google-cn",
          "apple-cn",
          "proxy",
          "telegram-ip"
        ],
        "action": "route",
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
      "secret": "CUG_hb0015",
      "default_mode": "rule"
    }
  }
}
"#;
