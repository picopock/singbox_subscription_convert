mod clash;
mod cli;
mod singbox;
mod template;

use clap::Parser;
use reqwest::{blocking::Client, header};
use serde_json::{from_str as from_json_str, to_string_pretty};
use serde_yaml::from_str;
use singbox::outbound::{selector::Selector, urltest::Urltest, vmess::Vmess, Outbound};
use std::fs::{self, File};
use std::io::Write;
use std::path;

fn main() {
    let mut args: cli::Args = cli::Args::parse();
    args.url.query_pairs_mut().append_pair("flag", "clash");

    // Download the YAML data
    let response = Client::new().get(args.url).header(header::USER_AGENT, String::from("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")).send().unwrap();
    let yaml_data = response.text().unwrap();

    // Parse the YAML data into a Rust struct
    let clash_config: clash::Config = from_str(&yaml_data).unwrap();
    let proxies: &Vec<clash::proxy::Proxy> = &clash_config.proxies;

    let outbound_tags: Vec<String> = proxies
        .into_iter()
        .map(|proxy| proxy.name.to_string())
        .collect();
    let mut vmess_list: Vec<Outbound> = proxies
        .into_iter()
        .map(|proxy| {
            Outbound::Vmess(Vmess {
                tag: proxy.name.to_string(),
                server: proxy.server.to_string(),
                server_port: proxy.port,
                uuid: proxy.uuid.to_string(),
                security: Some(String::from("auto")),
                ..Vmess::default()
            })
        })
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

    // add vmess outbounds to singbox outbounds
    singbox_config.outbounds.append(&mut vmess_list);

    // update urltest outbounds with outbound_tags
    if let Some(Outbound::Urltest(Urltest { outbounds, .. })) = singbox_config.outbounds.get_mut(4)
    {
        // 修改 outbounds 的值
        *outbounds = outbound_tags.clone();
    }

    // update selector outbounds with outbound_tags
    if let Some(Outbound::Selector(Selector {
        outbounds, default, ..
    })) = singbox_config.outbounds.get_mut(5)
    {
        // 修改 outbounds 的值
        *outbounds = outbound_tags.clone();
        *default = Some(outbound_tags[0].clone());
    }

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
