use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use lsp_server::{Connection, ErrorCode, Message, Notification, Request, Response};
use lsp_types::{
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DidSaveTextDocumentParams, DocumentSymbolParams, DocumentSymbolResponse, GotoDefinitionParams,
    Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, Location,
    MarkupContent, MarkupKind, OneOf, Position, Range, ServerCapabilities, SymbolInformation,
    SymbolKind, TextDocumentSyncCapability, TextDocumentSyncKind, Uri,
};

#[derive(Clone, Debug)]
struct Definition {
    label: String,
    path: PathBuf,
    line: u32,
    start: u32,
    end: u32,
    doc: String,
    body: String,
    global: bool,
}

struct State {
    root: Option<PathBuf>,
    index: HashMap<String, Vec<Definition>>,
    documents: HashMap<Uri, String>,
}

impl State {
    fn new(root: Option<PathBuf>) -> Self {
        Self {
            root,
            index: HashMap::new(),
            documents: HashMap::new(),
        }
    }

    fn index_workspace(&mut self) {
        let Some(root) = self.root.clone() else {
            return;
        };
        let mut files = Vec::new();
        collect_lap_files(&root, &mut files);
        for path in files {
            if let Ok(text) = fs::read_to_string(&path) {
                self.index_file(&path, &text);
            }
        }
    }

    fn index_file(&mut self, path: &Path, text: &str) {
        self.remove_file(path);
        for definition in definitions(path, text) {
            self.index
                .entry(definition.label.clone())
                .or_default()
                .push(definition);
        }
    }

    fn remove_file(&mut self, path: &Path) {
        for definitions in self.index.values_mut() {
            definitions.retain(|definition| definition.path != path);
        }
        self.index.retain(|_, definitions| !definitions.is_empty());
    }

    fn definition_for(&self, label: &str, path: &Path) -> Option<&Definition> {
        let definitions = self.index.get(label)?;
        definitions
            .iter()
            .find(|definition| definition.path == path && definition.global)
            .or_else(|| {
                definitions
                    .iter()
                    .find(|definition| definition.path == path)
            })
            .or_else(|| definitions.iter().find(|definition| definition.global))
            .or_else(|| definitions.first())
    }
}

fn collect_lap_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if matches!(
            name.as_ref(),
            "target" | ".git" | "node_modules" | "grammars" | "tree-sitter-lap"
        ) {
            continue;
        }
        if path.is_dir() {
            collect_lap_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "lap") {
            files.push(path);
        }
    }
}

fn definitions(path: &Path, text: &str) -> Vec<Definition> {
    let lines: Vec<&str> = text.lines().collect();
    let mut definitions = Vec::new();
    let mut doc = Vec::new();
    for (line_number, line) in lines.iter().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("//") {
            doc.push(trimmed.trim_start_matches("//").trim().to_string());
            continue;
        }
        if trimmed.is_empty() {
            doc.clear();
            continue;
        }
        if let Some((label, start, end)) = binding_label(line) {
            definitions.push(Definition {
                label: label.to_string(),
                path: path.to_path_buf(),
                line: line_number as u32,
                start,
                end,
                doc: doc.join("\n"),
                body: binding_body(&lines, line_number),
                global: line.len() == trimmed.len(),
            });
        }
        doc.clear();
    }
    definitions
}

fn binding_body(lines: &[&str], start: usize) -> String {
    let mut body = String::new();
    let mut depth = 0i32;
    for line in &lines[start..] {
        if !body.is_empty() {
            body.push('\n');
        }
        body.push_str(line);
        let code = line.split("//").next().unwrap_or("");
        depth += code.matches(['{', '[', '(']).count() as i32;
        depth -= code.matches(['}', ']', ')']).count() as i32;
        if depth <= 0 {
            break;
        }
    }
    body
}

fn binding_label(line: &str) -> Option<(&str, u32, u32)> {
    let bytes = line.as_bytes();
    let mut index = 0;
    while index < bytes.len() && bytes[index].is_ascii_whitespace() {
        index += 1;
    }
    if index < bytes.len() && bytes[index] == b'*' {
        index += 1;
    }
    let start = index;
    while index < bytes.len()
        && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_' || bytes[index] == b'.')
    {
        index += 1;
    }
    if index == start {
        return None;
    }
    let label = &line[start..index];
    let mut cursor = index;
    while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
        cursor += 1;
    }
    if cursor < bytes.len() && bytes[cursor] == b':' {
        while cursor < bytes.len() && bytes[cursor] != b'=' {
            cursor += 1;
        }
    }
    if cursor < bytes.len() && bytes[cursor] == b'=' {
        Some((label, start as u32, index as u32))
    } else {
        None
    }
}

fn label_at(text: &str, position: Position) -> Option<String> {
    let line = text.lines().nth(position.line as usize)?;
    let characters: Vec<char> = line.chars().collect();
    let mut start = position.character as usize;
    let mut end = start;
    while start > 0 && is_label_character(characters.get(start - 1).copied()) {
        start -= 1;
    }
    while end < characters.len() && is_label_character(characters.get(end).copied()) {
        end += 1;
    }
    if start == end {
        return None;
    }
    Some(characters[start..end].iter().collect())
}

fn is_label_character(character: Option<char>) -> bool {
    character.is_some_and(|character| {
        character.is_ascii_alphanumeric() || character == '_' || character == '.'
    })
}

fn uri_to_path(uri: &Uri) -> Option<PathBuf> {
    url::Url::parse(uri.as_str()).ok()?.to_file_path().ok()
}

fn path_to_uri(path: &Path) -> Option<Uri> {
    url::Url::from_file_path(path)
        .ok()?
        .as_str()
        .parse::<Uri>()
        .ok()
}

fn location(definition: &Definition) -> Option<Location> {
    let uri = path_to_uri(&definition.path)?;
    Some(Location {
        uri,
        range: Range {
            start: Position {
                line: definition.line,
                character: definition.start,
            },
            end: Position {
                line: definition.line,
                character: definition.end,
            },
        },
    })
}

fn hover(definition: &Definition) -> Hover {
    let mut value = format!("```lap\n{}\n```", definition.body);
    if !definition.doc.is_empty() {
        value.push_str("\n\n");
        value.push_str(&definition.doc);
    }
    Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value,
        }),
        range: None,
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (connection, io_threads) = Connection::stdio();
    let (id, params) = connection.initialize_start()?;
    let init: InitializeParams = serde_json::from_value(params)?;
    let root = init
        .workspace_folders
        .as_ref()
        .and_then(|folders| folders.first())
        .and_then(|folder| uri_to_path(&folder.uri))
        .or_else(|| {
            #[allow(deprecated)]
            init.root_uri.as_ref().and_then(uri_to_path)
        })
        .or_else(|| {
            #[allow(deprecated)]
            init.root_path.clone().map(PathBuf::from)
        });
    let capabilities = ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        ..Default::default()
    };
    connection.initialize_finish(
        id,
        serde_json::json!({
            "capabilities": capabilities,
            "serverInfo": { "name": "lapzt", "version": env!("CARGO_PKG_VERSION") }
        }),
    )?;

    let mut state = State::new(root);
    state.index_workspace();

    for message in &connection.receiver {
        match message {
            Message::Request(request) => {
                if connection.handle_shutdown(&request)? {
                    break;
                }
                handle_request(&connection, &mut state, request)?;
            }
            Message::Notification(notification) => {
                handle_notification(&mut state, notification);
            }
            Message::Response(_) => {}
        }
    }
    drop(connection);
    io_threads.join()?;
    Ok(())
}

fn handle_request(
    connection: &Connection,
    state: &mut State,
    request: Request,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let response = match request.method.as_str() {
        "textDocument/hover" => {
            let params: HoverParams = serde_json::from_value(request.params)?;
            let uri = params.text_document_position_params.text_document.uri;
            let position = params.text_document_position_params.position;
            let result = document_text(state, &uri)
                .and_then(|text| label_at(&text, position))
                .and_then(|label| {
                    let path = uri_to_path(&uri)?;
                    state.definition_for(&label, &path).map(hover)
                });
            Response::new_ok(request.id, result)
        }
        "textDocument/definition" => {
            let params: GotoDefinitionParams = serde_json::from_value(request.params)?;
            let uri = params.text_document_position_params.text_document.uri;
            let position = params.text_document_position_params.position;
            let result = document_text(state, &uri)
                .and_then(|text| label_at(&text, position))
                .and_then(|label| {
                    let path = uri_to_path(&uri)?;
                    state.definition_for(&label, &path).and_then(location)
                });
            Response::new_ok(request.id, result)
        }
        "textDocument/documentSymbol" => {
            let params: DocumentSymbolParams = serde_json::from_value(request.params)?;
            let symbols = uri_to_path(&params.text_document.uri)
                .map(|path| document_symbols(state, &path))
                .unwrap_or_default();
            Response::new_ok(request.id, DocumentSymbolResponse::Flat(symbols))
        }
        _ => Response::new_err(
            request.id,
            ErrorCode::MethodNotFound as i32,
            format!("unsupported method: {}", request.method),
        ),
    };
    connection.sender.send(Message::Response(response))?;
    Ok(())
}

fn handle_notification(state: &mut State, notification: Notification) {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            if let Ok(params) =
                serde_json::from_value::<DidOpenTextDocumentParams>(notification.params)
            {
                let uri = params.text_document.uri;
                let text = params.text_document.text;
                if let Some(path) = uri_to_path(&uri) {
                    state.index_file(&path, &text);
                }
                state.documents.insert(uri, text);
            }
        }
        "textDocument/didChange" => {
            if let Ok(params) =
                serde_json::from_value::<DidChangeTextDocumentParams>(notification.params)
            {
                let uri = params.text_document.uri;
                if let Some(change) = params.content_changes.into_iter().last() {
                    if let Some(path) = uri_to_path(&uri) {
                        state.index_file(&path, &change.text);
                    }
                    state.documents.insert(uri, change.text);
                }
            }
        }
        "textDocument/didSave" => {
            if let Ok(params) =
                serde_json::from_value::<DidSaveTextDocumentParams>(notification.params)
            {
                let uri = params.text_document.uri;
                if let Some(path) = uri_to_path(&uri)
                    && let Ok(text) = fs::read_to_string(&path)
                {
                    state.index_file(&path, &text);
                    state.documents.insert(uri, text);
                }
            }
        }
        "textDocument/didClose" => {
            if let Ok(params) =
                serde_json::from_value::<DidCloseTextDocumentParams>(notification.params)
            {
                state.documents.remove(&params.text_document.uri);
            }
        }
        _ => {}
    }
}

fn document_text(state: &State, uri: &Uri) -> Option<String> {
    if let Some(text) = state.documents.get(uri) {
        return Some(text.clone());
    }
    let path = uri_to_path(uri)?;
    fs::read_to_string(path).ok()
}

fn document_symbols(state: &State, path: &Path) -> Vec<SymbolInformation> {
    let mut symbols = Vec::new();
    for definition in state.index.values().flatten() {
        if definition.path != path || !definition.global {
            continue;
        }
        let Some(location) = location(definition) else {
            continue;
        };
        #[allow(deprecated)]
        symbols.push(SymbolInformation {
            name: definition.label.clone(),
            kind: SymbolKind::FUNCTION,
            tags: None,
            deprecated: None,
            location,
            container_name: None,
        });
    }
    symbols.sort_by_key(|symbol| {
        (
            symbol.location.range.start.line,
            symbol.location.range.start.character,
        )
    });
    symbols
}
