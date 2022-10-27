use std::{
    fs::{self, File},
    path::Path,
};

use anyhow::Result;
use flate2::read::GzDecoder;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, Http, LapcePlugin, PLUGIN_RPC,
};
use lapce_vue::config::get_language_server_init_options;
use serde::Deserialize;
use serde_json::Value;
use tar_wasi::Archive;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: None,
        // glob pattern
        pattern: Some(String::from("*.{vue,ts,js,tsx,jsx}")),
        // like file:
        scheme: None,
    }];

    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
        .and_then(|server_path| {
            if !server_path.is_empty() {
                Some(server_path)
            } else {
                None
            }
        });

    let language_init_option = get_language_server_init_options(params.root_uri);

    let server_args = vec!["--stdio".to_string()];

    if let Some(server_path) = server_path {
        PLUGIN_RPC.start_lsp(
            Url::parse(&format!("urn:{}", server_path))?,
            server_args,
            document_selector,
            language_init_option,
        );
        PLUGIN_RPC.stderr("自定义 server_path 启动lapce-vue成功");

        return Ok(());
    }

    download_volar()?;

    let volt_uri = std::env::var("VOLT_URI")?;
    let server_path = Url::parse(&volt_uri)
        .unwrap()
        .join("vue-language-server")
        .unwrap();

    PLUGIN_RPC.stderr(&format!("server_path：{}", server_path));
    PLUGIN_RPC.start_lsp(
        server_path,
        server_args.clone(),
        document_selector.clone(),
        language_init_option,
    );
    PLUGIN_RPC.stderr("启动lapce-vue");

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                let _ = initialize(params);
            }
            _ => {}
        }
    }
}

#[derive(Deserialize, Debug)]
struct VoltConfig {
    version: String,
}

fn download_volar() -> Result<bool> {
    let volt_path = Path::new("volt.toml");

    let volt_str = fs::read_to_string(volt_path)?;

    let volt_toml: VoltConfig = toml::from_str(&volt_str)?;

    let arch = match std::env::var("VOLT_ARCH").as_deref() {
        Ok("x86_64") => "x64",
        Ok("aarch64") => "arm64",
        _ => panic!("unknow arch"),
    };
    let os = match std::env::var("VOLT_OS").as_deref() {
        Ok("linux") => "linux",
        Ok("macos") => "macos",
        Ok("windows") => "win",
        _ => panic!("unknow os"),
    };

    let lapce_volar_base_name = format!("vue-language-server");

    let lapce_volar_name = format!("{}-{}-{}", lapce_volar_base_name, os, arch);
    let lapce_volar_gz_path_name = format!("{}.tar.gz", &lapce_volar_name.clone());
    let lapce_volar_gz_path = Path::new(&lapce_volar_gz_path_name);

    if !lapce_volar_gz_path.exists() {
        let volt_download_url = format!(
            "https://github.com/xiaoxin-sky/lapce-vue/releases/download/v{}/{}",
            &volt_toml.version, &lapce_volar_gz_path_name,
        );
        PLUGIN_RPC.stderr(&format!("下载地址_{}", volt_download_url));

        let mut resp = Http::get(&volt_download_url)?;
        let body = resp.body_read_all()?;
        fs::write(&lapce_volar_gz_path, body)?;
    } else {
        PLUGIN_RPC.stderr("压缩包已存在,跳过下载");
    }

    let tar_gz = File::open(&lapce_volar_gz_path_name);
    match tar_gz {
        Ok(tar) => {
            let tar = GzDecoder::new(tar);
            let mut archive = Archive::new(tar);

            let res = archive.unpack(".");

            PLUGIN_RPC.stderr(&format!("{:#?}", res));
            fs::rename(lapce_volar_name, lapce_volar_base_name)?;
        }
        Err(err) => {
            PLUGIN_RPC.stderr(&format!("{}", err));
        }
    }

    Ok(true)
}
