use std::fs;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::convert::Infallible;
use axum::{
    Router,
    routing::post,
    body::Body,
    extract::{State, Request},
    response::{Response, IntoResponse},
    http::{StatusCode, HeaderMap},
};
use serde_json::Value;
use anyhow::{Result, Context};
use futures_util::stream::StreamExt;

/// Shared state for the HTTP proxy.
#[derive(Clone)]
pub struct ProxyState {
    pub project_root: PathBuf,
    pub kit_root: PathBuf,
}

impl ProxyState {
    pub fn new(project_root: PathBuf, kit_root: PathBuf) -> Self {
        Self {
            project_root,
            kit_root,
        }
    }

    /// Reads and consolidates `.context.md` and `.user.md` rules.
    pub fn get_consolidated_rules(&self) -> String {
        let mut rules = String::new();
        rules.push_str("=== SYSTEM ACTIVE BEHAVIOR CONTEXT (BRAINFORGE) ===\n");
        rules.push_str("You must strictly adhere to the following project standards, architectural rules, and stack constraints.\n\n");
        
        let memory_dir = self.kit_root.join("memory");
        
        if let Ok(c) = fs::read_to_string(memory_dir.join(".context.md")) {
            rules.push_str("--- PROJECT CONTEXT & STACK ---\n");
            rules.push_str(&c);
            rules.push_str("\n");
        }
        
        if let Ok(u) = fs::read_to_string(memory_dir.join(".user.md")) {
            rules.push_str("--- USER ARCHITECTURAL RULES & STACKS ---\n");
            rules.push_str(&u);
            rules.push_str("\n");
        }
        
        rules.push_str("=== END OF ACTIVE BEHAVIOR CONTEXT (BRAINFORGE) ===\n");
        rules
    }
}

/// Helper function to perform robust inline log compression (software-based RTK)
fn compress_prompt_logs(text: &str) -> String {
    if text.len() > 3000 && (text.contains("Exception") || text.contains("Traceback") || text.contains("at ") || text.contains("compiler error") || text.contains("Build failed")) {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() > 60 {
            let mut compressed = String::new();
            compressed.push_str("[... BrainForge RTK Local: Logs compactados de forma eficiente para evitar saturação da Janela de Contexto ...]\n");
            for &line in lines.iter().take(25) {
                compressed.push_str(line);
                compressed.push_str("\n");
            }
            let omitted_count = lines.len() - 50;
            compressed.push_str(&format!("\n\n[... Omitidas {} linhas de logs repetitivos e avisos redundantes do compilador por eficiência de tokens ...]\n\n\n", omitted_count));
            for &line in lines.iter().skip(lines.len() - 25) {
                compressed.push_str(line);
                compressed.push_str("\n");
            }
            return compressed;
        }
    }
    text.to_string()
}

/// Starts the local HTTP API Proxy server.
pub async fn start_proxy(project_root: PathBuf, port: u16) -> Result<()> {
    // Resolve project kit paths
    let paths = crate::kit::KitPaths::resolve(&project_root, None)
        .context("Failed to resolve project kit paths for proxy")?;
        
    let state = ProxyState::new(paths.project_root, paths.kit_root);
    
    let app = Router::new()
        .route("/v1/chat/completions", post(handle_openai))
        .route("/v1/messages", post(handle_anthropic))
        .fallback(handle_fallback)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await
        .context(format!("Failed to bind proxy server to port {}", port))?;
        
    println!("[brainforge] Gateway Universal de IA ativo em http://127.0.0.1:{}", port);
    
    axum::serve(listener, app).await
        .context("Error running proxy server loop")?;
        
    Ok(())
}

/// Handler for OpenAI completions (`/v1/chat/completions`).
async fn handle_openai(
    State(state): State<ProxyState>,
    headers: HeaderMap,
    req: Request,
) -> Result<Response, Infallible> {
    let client = reqwest::Client::new();
    let upstream_url = "https://api.openai.com/v1/chat/completions";
    
    let (_parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    
    let mut payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(json) => json,
        Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    
    // Inject active rules as static System Prompt
    let consolidated_rules = state.get_consolidated_rules();
    
    if let Some(messages) = payload.get_mut("messages").and_then(|m| m.as_array_mut()) {
        // Apply RTK log compression on user inputs to prevent window clogging
        for msg in messages.iter_mut() {
            if let Some(content) = msg.get_mut("content") {
                if let Some(txt) = content.as_str() {
                    let optimized = compress_prompt_logs(txt);
                    if optimized != txt {
                        *content = Value::String(optimized);
                    }
                }
            }
        }

        // Find or prepend system message
        let mut system_msg_idx = None;
        for (i, msg) in messages.iter().enumerate() {
            if msg.get("role").and_then(|r| r.as_str()) == Some("system") {
                system_msg_idx = Some(i);
                break;
            }
        }
        
        if let Some(idx) = system_msg_idx {
            if let Some(content) = messages[idx].get_mut("content").and_then(|c| c.as_str()) {
                let merged = format!("{}\n\n{}", consolidated_rules, content);
                messages[idx]["content"] = Value::String(merged);
            }
        } else {
            let new_sys = serde_json::json!({
                "role": "system",
                "content": consolidated_rules
            });
            messages.insert(0, new_sys);
        }
    }
    
    // Forward request upstream
    let upstream_req = match build_upstream_request(&client, upstream_url, &headers, payload) {
        Ok(r) => r,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };
    
    match forward_upstream(&client, upstream_req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(StatusCode::BAD_GATEWAY.into_response()),
    }
}

/// Handler for Anthropic Claude messages (`/v1/messages`).
async fn handle_anthropic(
    State(state): State<ProxyState>,
    headers: HeaderMap,
    req: Request,
) -> Result<Response, Infallible> {
    let client = reqwest::Client::new();
    let upstream_url = "https://api.anthropic.com/v1/messages";
    
    let (_parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    
    let mut payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(json) => json,
        Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    
    // Inject active rules into Anthropic System prompt field
    let consolidated_rules = state.get_consolidated_rules();
    
    // Apply RTK log compression on user inputs
    if let Some(messages) = payload.get_mut("messages").and_then(|m| m.as_array_mut()) {
        for msg in messages.iter_mut() {
            if let Some(content) = msg.get_mut("content") {
                if let Some(txt) = content.as_str() {
                    let optimized = compress_prompt_logs(txt);
                    if optimized != txt {
                        *content = Value::String(optimized);
                    }
                }
            }
        }
    }

    if let Some(sys_field) = payload.get_mut("system") {
        if let Some(original) = sys_field.as_str() {
            let merged = format!("{}\n\n{}", consolidated_rules, original);
            *sys_field = Value::String(merged);
        }
    } else {
        payload["system"] = Value::String(consolidated_rules);
    }
    
    // Forward request upstream
    let upstream_req = match build_upstream_request(&client, upstream_url, &headers, payload) {
        Ok(r) => r,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };
    
    match forward_upstream(&client, upstream_req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(StatusCode::BAD_GATEWAY.into_response()),
    }
}

/// Fallback proxy for all other endpoints (transparent pass-through).
async fn handle_fallback(
    State(_state): State<ProxyState>,
    headers: HeaderMap,
    req: Request,
) -> Result<Response, Infallible> {
    let client = reqwest::Client::new();
    let path = req.uri().path();
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();
    
    // Determine host based on path prefix
    let (host, final_path) = if path.starts_with("/v1/messages") || path.contains("anthropic") {
        ("https://api.anthropic.com", path)
    } else {
        ("https://api.openai.com", path)
    };
    
    let upstream_url = format!("{}{}{}", host, final_path, query);
    
    let (_parts, body) = req.into_parts();
    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
    };
    
    let payload: Value = match serde_json::from_slice(&body_bytes) {
        Ok(json) => json,
        Err(_) => {
            // If body is not json, forward raw body bytes
            let upstream_req = client.post(&upstream_url)
                .headers(clone_headers(&headers))
                .body(body_bytes);
            return match forward_upstream(&client, upstream_req).await {
                Ok(res) => Ok(res),
                Err(_) => Ok(StatusCode::BAD_GATEWAY.into_response()),
            };
        }
    };
    
    let upstream_req = match build_upstream_request(&client, &upstream_url, &headers, payload) {
        Ok(r) => r,
        Err(_) => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
    };
    
    match forward_upstream(&client, upstream_req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(StatusCode::BAD_GATEWAY.into_response()),
    }
}

/// Builds reqwest request builder from incoming headers and json body.
fn build_upstream_request(
    client: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    body: Value,
) -> Result<reqwest::RequestBuilder> {
    let mut builder = client.post(url)
        .headers(clone_headers(headers))
        .json(&body);
        
    // Standardize Content-Type for safety
    builder = builder.header("content-type", "application/json");
    
    Ok(builder)
}

/// Clones header maps cleanly omitting host header.
fn clone_headers(headers: &HeaderMap) -> HeaderMap {
    let mut clean = HeaderMap::new();
    for (k, v) in headers.iter() {
        if k != "host" && k != "content-length" && k != "content-type" {
            clean.insert(k.clone(), v.clone());
        }
    }
    clean
}

/// Forwards request upstream handling Server-Sent Events (SSE) stream cleanly.
async fn forward_upstream(_client: &reqwest::Client, req: reqwest::RequestBuilder) -> Result<Response> {
    let res = req.send().await?;
    let status = res.status();
    let res_headers = res.headers().clone();
    
    let mut response_builder = Response::builder().status(status.as_u16());
    
    // Copy headers from upstream response to downstream response
    if let Some(headers_ref) = response_builder.headers_mut() {
        for (k, v) in res_headers.iter() {
            headers_ref.insert(k.clone(), v.clone());
        }
    }
    
    let stream = res.bytes_stream().map(|chunk| {
        chunk.map_err(|e| {
            std::io::Error::new(std::io::ErrorKind::Other, e)
        })
    });
    
    let body = Body::from_stream(stream);
    let final_res = response_builder.body(body)?;
    
    Ok(final_res)
}
