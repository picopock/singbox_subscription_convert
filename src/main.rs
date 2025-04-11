mod clash;
mod cli;
mod singbox;
mod template;

use clap::Parser;
use clash::proxy::Proxy;
use reqwest::{blocking::Client, header};
use serde_json::{from_str as from_json_str, to_string_pretty};
use serde_yaml::from_str;
use singbox::outbound::{
    http, selector::Selector, shadowsocks, socks, trojan, urltest::Urltest, vmess::Vmess, Outbound,
};
use std::fs::{self, File};
use std::io::Write;
use std::path;

pub fn extra_name(outbound: &Proxy) -> String {
    match outbound {
        Proxy::Ss(ss) => ss.name.to_string(),
        Proxy::Ssr(ssr) => ssr.name.to_string(),
        Proxy::Vmess(vmess) => vmess.name.to_string(),
        Proxy::Socks5(socks5) => socks5.name.to_string(),
        Proxy::Http(http) => http.name.to_string(),
        Proxy::Snell(snell) => snell.name.to_string(),
        Proxy::Trojan(trojan) => trojan.name.to_string(),
    }
}

pub fn transform_outbound_protocol(outbound: &Proxy) -> Outbound {
    match outbound {
        Proxy::Ss(ss) => {
            return Outbound::Shadowsocks(shadowsocks::Shadowsocks {
                tag: ss.name.to_string(),
                server: ss.server.to_string(),
                server_port: ss.port,
                method: ss.cipher.to_string(),
                password: ss.password.to_string(),
                ..shadowsocks::Shadowsocks::default()
            });
        }
        Proxy::Ssr(ssr) => {
            return Outbound::Shadowsocks(shadowsocks::Shadowsocks {
                tag: ssr.name.to_string(),
                server: ssr.server.to_string(),
                server_port: ssr.port,
                password: ssr.password.to_string(),
                bind_interface: ssr.interface_name.clone(),
                routing_mark: ssr.routing_mark,
                ..shadowsocks::Shadowsocks::default()
            });
        }
        Proxy::Vmess(vmess) => {
            return Outbound::Vmess(Vmess {
                tag: vmess.name.to_string(),
                server: vmess.server.to_string(),
                server_port: vmess.port,
                uuid: vmess.uuid.clone(),
                security: Some(String::from("auto")),
                ..Vmess::default()
            });
        }
        Proxy::Socks5(socks5) => {
            return Outbound::Socks(socks::Socks {
                tag: socks5.name.to_string(),
                server: socks5.server.to_string(),
                server_port: socks5.port,
                username: socks5.username.clone(),
                password: socks5.password.clone(),
                bind_interface: socks5.interface_name.clone(),
                routing_mark: socks5.routing_mark,
                ..socks::Socks::default()
            });
        }
        Proxy::Http(http) => {
            return Outbound::Http(http::Http {
                tag: http.name.to_string(),
                server: http.server.to_string(),
                server_port: http.port,
                username: http.username.clone(),
                password: http.password.clone(),
                bind_interface: http.interface_name.clone(),
                routing_mark: http.routing_mark,
                ..http::Http::default()
            });
        }
        Proxy::Snell(snell) => {
            return Outbound::Vmess(Vmess {
                tag: snell.name.to_string(),
                server: snell.server.to_string(),
                server_port: snell.port,
                security: Some(String::from("auto")),
                ..Vmess::default()
            });
        }
        Proxy::Trojan(trojan) => {
            return Outbound::Trojan(trojan::Trojan {
                tag: trojan.name.to_string(),
                server: trojan.server.to_string(),
                server_port: trojan.port,
                password: trojan.password.clone(),
                network: trojan.network.clone(),
                bind_interface: trojan.interface_name.clone(),
                routing_mark: trojan.routing_mark,
                ..trojan::Trojan::default()
            });
        }
    }
}

fn process_outbounds(vec: &mut Vec<String>, outbounds: &[String]) {
    let mut index = 0;
    while index < vec.len() {
        let current = &vec[index];
        if current == "{{ALL-TAG}}" {
            vec.splice(index..index + 1, outbounds.iter().cloned());
        } else if current.starts_with("{{") && current.ends_with("}}") {
            let inner = &current[2..current.len() - 2];
            let mut exclude_rules: Vec<&str> = Vec::new();
            let mut include_rules: Vec<&str> = Vec::new();

            if inner.contains("EXCLUDE-TAG:") {
                let start = inner.find("EXCLUDE-TAG:").unwrap() + "EXCLUDE-TAG:".len();
                let end = inner[start..]
                    .find(';')
                    .unwrap_or_else(|| inner.len() - start);
                exclude_rules = inner[start..start + end].split(',').collect();
            }

            if inner.contains("INCLUDE-TAG:") {
                let start = inner.find("INCLUDE-TAG:").unwrap() + "INCLUDE-TAG:".len();
                let end = inner[start..]
                    .find(';')
                    .unwrap_or_else(|| inner.len() - start);
                include_rules = inner[start..start + end].split(',').collect();
            }

            let mut matches = Vec::new();
            for outbound in outbounds {
                let should_exclude = exclude_rules.iter().any(|rule| outbound.contains(rule));
                let should_include = include_rules.is_empty()
                    || include_rules.iter().any(|rule| outbound.contains(rule));

                if should_include && !should_exclude {
                    matches.push(outbound.clone());
                }
            }

            vec.splice(index..index + 1, matches.into_iter());
        } else {
            index += 1;
        }
    }
}

fn process_default(default: &mut Option<String>, outbounds: &[String]) {
    // 检查 default 是否为 None 且 outbounds 不为空
    if (default.is_none() || default.as_ref().map_or(false, |s| s.is_empty()))
        && !outbounds.is_empty()
    {
        *default = outbounds.first().cloned();
    }

    // 如果 default 有值，继续处理
    if let Some(default_rule) = default.as_mut() {
        if default_rule.starts_with("{{") && default_rule.ends_with("}}") {
            let inner = &default_rule[2..default_rule.len() - 2];
            let mut exclude_rules: Vec<&str> = Vec::new();
            let mut include_rules: Vec<&str> = Vec::new();

            // 安全地解析 EXCLUDE-TAG 规则
            if let Some(start_index) = inner.find("EXCLUDE-TAG:") {
                let start = start_index + "EXCLUDE-TAG:".len();
                if let Some(end_index) = inner[start..].find(';') {
                    let end = start + end_index;
                    exclude_rules = inner[start..end].split(',').collect();
                } else {
                    exclude_rules = inner[start..].split(',').collect();
                }
            }

            // 安全地解析 INCLUDE-TAG 规则
            if let Some(start_index) = inner.find("INCLUDE-TAG:") {
                let start = start_index + "INCLUDE-TAG:".len();
                if let Some(end_index) = inner[start..].find(';') {
                    let end = start + end_index;
                    include_rules = inner[start..end].split(',').collect();
                } else {
                    include_rules = inner[start..].split(',').collect();
                }
            }

            let mut matches = Vec::new();
            for outbound in outbounds {
                let should_exclude = exclude_rules.iter().any(|rule| outbound.contains(rule));
                let should_include = include_rules.is_empty()
                    || include_rules.iter().any(|rule| outbound.contains(rule));

                if should_include && !should_exclude {
                    matches.push(outbound.clone());
                }
            }

            // 如果有匹配项，更新 default
            if !matches.is_empty() {
                *default = Some(matches[0].clone());
            }
        }
    }
}

fn main() {
    let mut args: cli::Args = cli::Args::parse();
    args.url.query_pairs_mut().append_pair("flag", "clash");

    // Download the YAML data
    let response = Client::new().get(args.url).header(header::USER_AGENT, String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")).send().unwrap();
    let yaml_data = response.text().unwrap();

    // Parse the YAML data into a Rust struct
    let clash_config: clash::Config = from_str(&yaml_data).unwrap();
    let proxies: &Vec<clash::proxy::Proxy> = &clash_config.proxies;

    let outbound_tags: Vec<String> = proxies.into_iter().map(extra_name).collect();
    let mut vmess_list: Vec<Outbound> = proxies
        .into_iter()
        .map(transform_outbound_protocol)
        .collect();

    // create singbox config from template
    let mut template_str: String = template::TEMPLATE.to_string();
    if let Some(template_path) = args.template {
        if !template_path.exists() {
            println!("Template file not found! Use default template instead.");
        } else if template_path.is_dir() {
            if template_path.join("template.json").exists() {
                template_str = fs::read_to_string(template_path.join("template.json"))
                    .expect("READ TEMPLATE ERROR");
            } else {
                println!("Can not find the template! Use default template instead.");
            }
        } else {
            template_str = fs::read_to_string(&template_path).expect("READ TEMPLATE ERROR");
        }
    }
    let mut singbox_config: singbox::Config = from_json_str(template_str.as_str()).unwrap();

    for outbound in &mut singbox_config.outbounds {
        // update urltest outbounds with outbound_tags
        if let Outbound::Urltest(Urltest { outbounds, .. }) = outbound {
            process_outbounds(outbounds, &outbound_tags);
        }

        // update selector outbounds with outbound_tags
        if let Outbound::Selector(Selector {
            outbounds, default, ..
        }) = outbound
        {
            process_default(default, &outbound_tags);
            process_outbounds(outbounds, &outbound_tags);
        }
    }

    // add vmess outbounds to singbox outbounds
    singbox_config.outbounds.append(&mut vmess_list);

    // Convert the updated template to JSON
    let json_data = to_string_pretty(&singbox_config).unwrap();

    // Write the JSON data to config.json
    let mut output_path = args.output.unwrap_or(path::PathBuf::from("config.json"));
    if output_path.is_dir() {
        output_path.push("config.json");
    }
    let mut file = File::create(output_path).unwrap();
    file.write_all(json_data.as_bytes()).unwrap();

    println!("JSON data written to config.json");
}
