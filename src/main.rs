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
    // let second_language_feature_option =
    //     config::get_initialization_options(LanguageOptionEnum::second_language_feature);
    // let doc_language_feature_option =
    //     config::get_initialization_options(LanguageOptionEnum::document_feature);

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
        // "/Users/johnsonchu/Desktop/GitHub/volar/packages/vue-language-server/bin/vue-language-server.js".to_string(),
        // "--stdio".to_string(),
        "--inspect".to_string(),
    ];

    let server_path =
        Url::parse("file:///Users/skymac/workplace/volar/packages/vue-language-server/bin/run.sh")?;

    PLUGIN_RPC.start_lsp(
        server_path.clone(),
        server_args.clone(),
        document_selector.clone(),
        main_language_feature_option,
    );

    // PLUGIN_RPC.start_lsp(
    //     server_path.clone(),
    //     server_args.clone(),
    //     document_selector.clone(),
    //     second_language_feature_option,
    // );
    // PLUGIN_RPC.start_lsp(
    //     server_path.clone(),
    //     server_args.clone(),
    //     document_selector.clone(),
    //     doc_language_feature_option,
    // );

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
