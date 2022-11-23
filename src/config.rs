use lapce_plugin::psp_types::lsp_types::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TypescriptSDK {
    tsdk: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum ServerMode {
    Semantic = 0,
    // PartialSemantic = 1, // not support yet
    Syntactic = 2,
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum DiagnosticModel {
    None = 0,
    Push = 1,
    Pull = 2,
}
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageServerInitializationOptions {
    typescript: TypescriptSDK,
    #[serde(skip_serializing_if = "Option::is_none")]
    serverMode: Option<ServerMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnosticModel: Option<DiagnosticModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    textDocumentSync: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cancellationPipeName: Option<String>,
}
pub fn get_language_server_init_options(root_url: Option<Url>) -> Option<Value> {
    let root_url = root_url.unwrap();
    let tsdk = root_url
        .join("node_modules/typescript/lib")
        .unwrap()
        .to_file_path()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let initialization_options = LanguageServerInitializationOptions {
        typescript: TypescriptSDK { tsdk },
        serverMode: None,
        diagnosticModel: None,
        textDocumentSync: None,
        cancellationPipeName: None,
    };
    serde_json::to_value(&initialization_options).ok()
}
