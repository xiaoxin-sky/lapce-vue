use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use lapce_vue::config::get_language_server_init_options;
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: Some(String::from("vue")),
        // glob pattern
        pattern: Some(String::from("**.vue")),
        // like file:
        scheme: None,
    }];
    let server_args = vec!["--stdio".to_string()];

    let volt_uri = std::env::var("VOLT_URI")?;
    let server_path = Url::parse(&volt_uri).unwrap().join("main.js").unwrap();
    let language_init_option = get_language_server_init_options(Url::parse(&volt_uri).ok());

    PLUGIN_RPC.start_lsp(
        server_path,
        server_args.clone(),
        document_selector.clone(),
        language_init_option,
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
