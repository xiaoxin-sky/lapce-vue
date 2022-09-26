use std::{fs, path::Path};

use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use lapce_vue::config::{self, LanguageOptionEnum};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let main_language_feature_option =
        config::get_initialization_options(LanguageOptionEnum::main_language_feature);
    let second_language_feature_option =
        config::get_initialization_options(LanguageOptionEnum::second_language_feature);
    let doc_language_feature_option =
        config::get_initialization_options(LanguageOptionEnum::document_feature);

    // PLUGIN_RPC.stderr(&format!(
    //     "lapce_params:{:#?}",
    //     params.initialization_options
    // ));
    // PLUGIN_RPC.stderr(&format!(
    //     "main_language_feature_option:{:#?}",
    //     main_language_feature_option
    // ));
    // PLUGIN_RPC.stderr(&format!(
    //     "second_language_feature_option:{:#?}",
    //     second_language_feature_option
    // ));
    // PLUGIN_RPC.stderr(&format!(
    //     "doc_language_feature_option:{:#?}",
    //     doc_language_feature_option
    // ));

    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: Some(String::from("vue")),
        // glob pattern
        pattern: Some(String::from("**.vue")),
        // like file:
        scheme: None,
    }];
    let mut server_args = vec![
        // "/Users/xiaoxin/node_modules/@volar/vue-language-server/out/nodeServer.js".to_string(),
        "/Users/johnsonchu/Desktop/GitHub/volar/packages/vue-language-server/bin/vue-language-server.js".to_string(),
        "--stdio".to_string(),
        "--inspect".to_string(),
    ];

    // Check for user specified LSP server path
    // ```
    // [lapce-plugin-name.lsp]
    // serverPath = "[path or filename]"
    // serverArgs = ["--arg1", "--arg2"]
    // ```
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lsp") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                        PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }
        }
    }

    // Download URL
    // let _ = format!("https://github.com/<name>/<project>/releases/download/<version>/{filename}");

    // see lapce_plugin::Http for available API to download files

    // let _ = match VoltEnvironment::operating_system().as_deref() {
    //     Ok("windows") => {
    //         format!("{}.exe", "[filename]")
    //     }
    //     _ => "[filename]".to_string(),
    // };

    // Plugin working directory
    // let volt_uri = VoltEnvironment::uri()?;
    let server_path = Url::parse("urn:vue-language-server")?;

    // if you want to use server from PATH
    // let server_path = Url::parse(&format!("urn:{filename}"))?;

    // Available language IDs
    // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    PLUGIN_RPC.start_lsp(
        server_path.clone(),
        server_args.clone(),
        document_selector.clone(),
        main_language_feature_option,
    );
    PLUGIN_RPC.start_lsp(
        server_path.clone(),
        server_args.clone(),
        document_selector.clone(),
        second_language_feature_option,
    );
    PLUGIN_RPC.start_lsp(
        server_path.clone(),
        server_args.clone(),
        document_selector.clone(),
        doc_language_feature_option,
    );

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
