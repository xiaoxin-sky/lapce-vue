use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
/// Ts server path
struct TypescriptPath {
    /**
     * Path to tsserverlibrary.js / tsserver.js / typescript.js
     * @example
     * '/usr/local/lib/node_modules/typescript/lib/tsserverlibrary.js' // use global typescript install
     * 'typescript/lib/tsserverlibrary.js' // if `typescript` exist in `@volar/vue-lannguage-server` itself node_modules directory
     * '../../../typescript/lib/tsserverlibrary.js' // relative path to @volar/vue-language-server/out/index.js
     */
    serverPath: String,
    /**
     * Path to lib/xxx/diagnosticMessages.generated.json
     * @example
     * '/usr/local/lib/node_modules/typescript/lib/ja/diagnosticMessages.generated.json' // use global typescript install
     * 'typescript/lib/ja/diagnosticMessages.generated.json' // if `typescript` exist in `@volar/vue-lannguage-server` itself node_modules directory
     * '../../../typescript/lib/ja/diagnosticMessages.generated.json' // relative path to @volar/vue-language-server/out/index.js
     */
    localizedPath: Option<String>,
}

/// both | kebabCase | pascalCase

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DefaultTagNameCase {
    both,
    kebabCase,
    pascalCase,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DefaultAttrNameCase {
    kebabCase,
    camelCase,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Completion {
    pub defaultTagNameCase: DefaultTagNameCase,
    pub defaultAttrNameCase: DefaultAttrNameCase,
    /**
     * {@link __requests.GetDocumentNameCasesRequest}
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getDocumentNameCasesRequest: Option<bool>,
    /**
     * {@link __requests.GetDocumentSelectionRequest}
     * */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub getDocumentSelectionRequest: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignoreTriggerCharacters: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeLens {
    showReferencesNotification: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaRequestService {
    pub getDocumentContentRequest: Option<bool>,
}
/**
 * typescript, html, css... language service will be create in server if this option is not null
 */
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    references: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    implementation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    definition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    typeDefinition: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callHierarchy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    renameFileRefactoring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signatureHelp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completion: Option<Completion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentHighlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentLink: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workspaceSymbol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codeLens: Option<CodeLens>,
    #[serde(skip_serializing_if = "Option::is_none")]
    semanticTokens: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    codeAction: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inlayHints: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    schemaRequestService: Option<SchemaRequestService>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowedLanguageIds: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selectionRange: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foldingRange: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    linkedEditingRange: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentSymbol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentColor: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentFormatting: Option<bool>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MainLanguageFeature {
    pub references: Option<bool>,
    pub implementation: Option<bool>,
    pub definition: Option<bool>,
    pub typeDefinition: Option<bool>,
    pub callHierarchy: Option<bool>,
    pub hover: Option<bool>,
    pub rename: Option<bool>,
    pub renameFileRefactoring: Option<bool>,
    pub signatureHelp: Option<bool>,
    pub codeAction: Option<bool>,
    pub workspaceSymbol: Option<bool>,
    pub completion: Completion,
    pub schemaRequestService: Option<SchemaRequestService>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct SecondLanguageFeature {
    documentHighlight: Option<bool>,
    documentLink: Option<bool>,
    codeLens: Option<CodeLens>,
    semanticTokens: Option<bool>,
    inlayHints: Option<bool>,
    diagnostics: Option<bool>,
    schemaRequestService: Option<SchemaRequestService>,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInitializationOptions {
    // textDocumentSync: TextDocumentSyncKind,
    typescript: TypescriptPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    languageFeatures: Option<LanguageFeatures>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentFeatures: Option<DocumentFeatures>,
}
pub fn get_main_language_feature() -> LanguageFeatures {
    LanguageFeatures {
        references: None,
        // 不存在
        implementation: None,
        definition: None,
        typeDefinition: None,
        callHierarchy: None,
        hover: None,
        rename: None,
        renameFileRefactoring: None,
        signatureHelp: None,
        codeAction: None,
        workspaceSymbol: None,
        completion: Some(Completion {
            defaultTagNameCase: DefaultTagNameCase::both,
            defaultAttrNameCase: DefaultAttrNameCase::kebabCase,
            getDocumentNameCasesRequest: Some(false),
            getDocumentSelectionRequest: Some(false),
            ignoreTriggerCharacters: None,
        }),
        schemaRequestService: None,
        documentHighlight: None,
        documentLink: None,
        codeLens: None,
        semanticTokens: None,
        inlayHints: None,
        diagnostics: None,
    }
}

pub fn get_second_language_feature() -> LanguageFeatures {
    LanguageFeatures {
        documentHighlight: None,
        documentLink: None,
        codeLens: None,
        semanticTokens: None,
        // 不存在
        inlayHints: None,
        diagnostics: None,
        schemaRequestService: None,
        references: None,
        implementation: None,
        definition: None,
        typeDefinition: None,
        callHierarchy: None,
        hover: None,
        rename: None,
        renameFileRefactoring: None,
        signatureHelp: None,
        completion: None,
        workspaceSymbol: None,
        codeAction: None,
    }
}

pub fn get_doc_feature() -> DocumentFeatures {
    DocumentFeatures {
        // allowedLanguageIds: vec![
        //     "vue".to_owned(),
        //     "javascript".to_owned(),
        //     "typescript".to_owned(),
        //     "javascriptreact".to_owned(),
        //     "typescriptreact".to_owned(),
        // ],
        allowedLanguageIds: None,
        selectionRange: None,
        foldingRange: None,
        linkedEditingRange: None,
        documentSymbol: None,
        documentColor: None,
        // 没有格式胡
        documentFormatting: None,
    }
}

pub enum LanguageOptionEnum {
    main_language_feature,
    second_language_feature,
    document_feature,
}

pub fn get_initialization_options(name: LanguageOptionEnum) -> Option<Value> {
    let language_features = match name {
        LanguageOptionEnum::main_language_feature => Some(get_main_language_feature()),
        LanguageOptionEnum::second_language_feature => Some(get_second_language_feature()),
        LanguageOptionEnum::document_feature => None,
    };
    let document_features = match name {
        LanguageOptionEnum::main_language_feature => None,
        LanguageOptionEnum::second_language_feature => None,
        LanguageOptionEnum::document_feature => Some(get_doc_feature()),
    };

    let initialization_options = ServerInitializationOptions {
        // textDocumentSync: TextDocumentSyncKind::Incremental,
        typescript: TypescriptPath {
            // serverPath: "/Users/skymac/node_modules/typescript/lib/tsserverlibrary.js".to_owned(),
            serverPath: "/Users/xiaoxin/Library/pnpm/global/5/.pnpm/typescript@4.7.4/node_modules/typescript/lib/tsserverlibrary.js".to_owned(),
            localizedPath: None, // localizedPath: Some(
                                 //     "/Users/skymac/node_modules/typescript/lib/zh-cn/diagnosticMessages.generated.json"
                                 //         .to_owned(),
                                 // ),
        },
        languageFeatures: language_features,
        documentFeatures: document_features,
    };
    serde_json::to_value(&initialization_options).ok()
}
